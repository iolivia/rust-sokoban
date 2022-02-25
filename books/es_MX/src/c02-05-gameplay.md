# Jugabilidad

El personaje del jugador puede moverse y empujar cajas en el mapa. Muchos juegos (¡pero no todos!) tienen algún tipo de objetivo que el jugador debe alcanzar. El objetivo de los juegos estilo Sokoban normalmente es empujar las cajas a sus lugares designados. No hay nada que le impida al jugador hacerlo en este momento, pero el juego tampoco está validando que sea exitoso. ¡El jugador podría alcanzar el objetivo sin darse cuenta! Actualicemos el juego para validar si se ha conseguido la meta.

Pensemos sobre lo que necesitaremos agregar a este juego para validar la condición de éxito y notificar al usuario cuando haya superado el nivel:

- Un `recurso` para dar seguimiento al estado del juego
    - ¿El juego en progreso ha sido completado?
    - ¿Cuántos movimientos ha hecho el jugador?
- Un `sistema` para revisar si el usuario ha completado el objetivo
- Un `sistema` para actualizar el número de movimientos hechos
- Interfaz de usuario para reportar el estado del juego

## Recurso Gameplay

Elegimos usar un `recurso` para dar seguimiento al estado del juego porque este estado no está asociado con ninguna entidad en específico. Empecemos definiendo un recurso `Gameplay`.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c02-05/src/resources.rs:38:43}}
```

`Gameplay` tiene dos campos: `state` y `moves_count`. Estos se utilizan para dar seguimiento al estado actual del juego (¿el juego está aún en progreso, o el jugador ha ganado?) y el número de movimientos realizados. `state` se describe por un `enum`, definido de esta forma:

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c02-05/src/resources.rs:17:20}}
```

El lector con vista aguda notará que utilizamos una macro para derivar el trait `Default` para `Gameplay`, pero no para el enum `GameplayState`. Si queremos utilizar `Gameplay` como un recurso, debe implementar `Default`.

¿Entonces, qué haremos? Ya que las macros de Rust no pueden derivar `Default` para los enum automáticamente, debemos implementar `Default` para `Gameplay` por nuestra cuenta.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c02-05/src/resources.rs:32:36}}
```

Habiendo definido el recurso, registrémoslo con el mundo:

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c02-05/src/resources.rs:12:15}}
```

Ahora, cuando el juego inicie, el recurso `Gameplay` lucirá así:

```rust
Gameplay {
    state: GameplayState::Playing,
    moves_count: 0
}
```

## Sistema contador de pasos

Podemos incrementar el campo `moves_count` de `Gameplay` para dar seguimiento al número de turnos utilizados.
Ya tenemos un sistema a cargo de la entrada del usuario en `InputSystem`, así que adaptémoslo para este propósito.

Ya que necesitamos mutar el recurso `Gameplay`, necesitamos registrarlo con `InputSystem` agregando `Write<'a, Gameplay>` a la definición del tipo `SystemData`.

```rust
// input_system.rs
{{#include ../../../code/rust-sokoban-c02-05/src/systems/input_system.rs:0:25}}
        ...
```

Como ya hemos hecho el trabajo de validar si un personaje se moverá en respuesta a la presión de una tecla, podemos utilizarlo también para determinar cuándo incrementar el contador de pasos.

```rust
// input_system.rs
        ...
{{#include ../../../code/rust-sokoban-c02-05/src/systems/input_system.rs:83:105}}
```

## Sistema Gameplay

A continuación, integremos este recurso con un nuevo `GameplayStateSystem`. Este sistema validará continuamente si todas las cajas tienen la misma posición que todos los puntos designados para ellas. Una vez que todas las cajas estén en dichos puntos, ¡el juego ha sido ganado!

Además de `Gameplay`, este sistema solo necesita acceso de lectura a los almacenamientos `Position`, `Box` y `BoxSpot`.

El sistema utiliza `Join` para crear un vector de los  almacenamientos `Box` y `Position`. Este vector se mapea a un hashmap que contiene la ubicación de cada caja en el tablero.

A continuación, el sistema utiliza nuevamente el método `Join` para crear un iterable de las entidades que tienen ambos componentes `BoxSpot` y `Position`. El sistema recorre este iterable.
Si todos los puntos designados para las cajas tienen una caja correspondiente en la misma posición, el juego termina y el jugador ha ganado. De lo contrario, el juego aún está en progreso.

```rust
// gameplay_state_system.rs
{{#include ../../../code/rust-sokoban-c02-05/src/systems/gameplay_state_system.rs::}}
```

Finalmente, ejecutemos el sistema gameplay en nuestro ciclo de actualización principal.

```rust
// main.rs
{{#include ../../../code/rust-sokoban-c02-05/src/main.rs:24:39}}
    // ...
{{#include ../../../code/rust-sokoban-c02-05/src/main.rs:63}}
```

## Interfaz de usuario para Gameplay

El último paso es dar retroalimentación al usuario para permitirle saber cuál es el estado del juego. Para ello se requiere un recurso para monitorear el estado y un sistema para actualizar el estado. Podemos adaptar el recurso `GameplayState` y el sistema `RenderingSystem` para esto.

Primero, implementaremos `Display` para `GameplayState` para que podamos renderizar el estado del juego como texto. Usaremos una expresión match para permitir que `GameplayState` renderice "Playing" (Jugando) o "Won" (Ganó).

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c02-05/src/resources.rs:21:30}}
```

A continuación, agregaremos un método `draw_text` a `RenderingSystem`, para que pueda imprimir `GameplayState` a la pantalla...

```rust
// rendering_systems.rs
{{#include ../../../code/rust-sokoban-c02-05/src/systems/rendering_system.rs:16:32}}
```

... y entonces agregaremos el recurso `Gameplay` a `RenderingSystem` para que podamos llamar a `draw_text`. `RenderingSystem` necesita poder leer el recurso `Gameplay`.

```rust
// rendering_system.rs
{{#include ../../../code/rust-sokoban-c02-05/src/systems/rendering_system.rs:35:71}}
```

En este punto, el juego dará retroalimentación básica al usuario:
- Cuenta el número de pasos
- Indica al jugador cuando ha ganado

Aquí tenemos cómo luce.

![Sokoban play](./images/moves.gif)


¡Hay bastantes más mejoras que se pueden hacer! 

> **_CODELINK:_**  Puedes ver el código completo de este ejemplo [aquí](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c02-05).