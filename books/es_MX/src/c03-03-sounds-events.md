# Sonidos y eventos

En esta sección trabajaremos en agregar efectos de sonido. De forma breve, queremos reproducir sonidos en estas circunstancias:
1. cuando el jugador encuentra una pared o un obstáculo - para hacerle saber que no puede continuar avanzando
1. cuando el jugador coloca una caja en el lugar correcto - como indicación de "lo has hecho correctamente"
1. cuando el jugador coloca una caja en un lugar incorrecto - como una indicación de que el movimiento fue incorrecto

En realidad reproducir audio no será muy difícil ya que ggez nos provee de esta habilidad, pero el problema más grande que tenemos ahora es que necesitamos determinar *cuándo* reproducir los sonidos.

Tomemos como ejemplo el colocar la caja en el lugar correcto. Probablemente podríamos utilizar nuestro sistema de estado del juego e iterar las cajas y las metas para determinar cuándo estamos en el estado deseado y reproducir el sonido. Pero no funcionará porque estaríamos iterando muchas veces por segundo, y siempre estaremos en este estado mientras la caja no se mueva, así que intentaríamos reproducir el sonido muchas veces, lo cual no es lo que queremos. Podríamos mantener algún estado para saber si en efecto nos encontramos jugando o no, pero tampoco parece correcto. El problema es que no podemos conseguirlo revisando el estado del juego de forma iterativa, necesitamos en su lugar tener un modelo reactivo donde se nos indique cuando algo ha ocurrido, y necesitamos tomar acción. Lo que acabamos de describir es un modelo de eventos, necesitamos disparar un evento cuando una caja se coloque en una meta, y entonces cuando recibamos este evento en otro punto necesitamos reproducir el sonido. El punto realmente bueno de esto es que podremos reutilizar este sistema de eventos para muchos otros propósitos.

## Infraestructura de eventos: Cómo
Comencemos discutiendo cómo implementaremos los eventos. No utilizaremos componentes ni entidades (aunque podríamos), en su lugar utilizaremos un recurso muy similar a la cola de entrada. Las partes del código que necesitan encolar eventos necesitarán acceso a este recurso, y entonces tendremos un sistema que procesa estos que procesa estos eventos y ejecuta las acciones apropiadas.

## Qué eventos
Discutamos con más detalle qué eventos necesitaremos:
1. el jugador golpea un obstáculo - este puede ser un evento que generamos desde el sistema de entrada cuando intentamos movernos pero no es posible hacerlo
1. caja colocada en una meta correcta/incorrecta - podemos modelar esto como un solo evento con una propiedad interna que nos indique si la combinación caja/meta es correcta - pensando un poco más sobre cómo podemos conseguir esto, podemos tener un evento cuando se mueve una entidad, y cuando recibimos este evento podemos revisar el id de la entidad que acaba de moverse para ver si es una caja y si está en una meta correcta/incorrecta/ninguna (este es un ejemplo de cómo crear una cadena de eventos - un evento a partir de otro evento)

## Tipos de eventos
Vayamos ahora a la implementación de los eventos. Utilizaremos un enum para definir varios tipos de eventos. Ahora, hemos utilizado enums anteriormente (para el tipo renderizable y para los colores de las cajas) pero esta vez aprovecharemos al máximo el poder de los enums de Rust. Una de las características más interesantes de ellas es que podemos adjuntar propiedades a cada variante de un enum.

Veamos nuestro enum de eventos.

```rust
// events.rs
{{#include ../../../code/rust-sokoban-c03-03/src/events.rs:13:23}}
```

Nota el segundo `EntityMoved` y el segundo `BoxPlacedOnSpot`. Esas son en realidad definiciones de estructuras donde podemos adjuntar propiedades. Veamos ahora esas estructuras.

```rust
// events.rs
{{#include ../../../code/rust-sokoban-c03-03/src/events.rs:1:11}}
```

## Recurso cola de eventos
Ahora agregaremos un recurso para la cola de eventos. Tendremos varios sistemas que escribirán a esta cola y un sistema (el sistema de eventos) que la consumirá. Básicamente es un modelo de múltiples productores y un solo consumidor.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:54:57}}
```

Y como siempre registremos este recurso.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:14:18}}
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:20}}
```

## Generando eventos
Ahora que tenemos una forma de encolar eventos, agreguemos los dos eventos que necesitamos en el sistema de entrada: EntityMoved y PlayerHitObstacle.

```rust
// input_system.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input_system.rs:1:42}}
                    // ...
                    // ...
{{#include ../../../code/rust-sokoban-c03-03/src/systems/input_system.rs:83:124}}
```

Omití parte del código en el archivo para facilitar la lectura, pero en realidad solo estamos agregando dos líneas en los lugares correctos.

## Consumiendo eventos - sistema de eventos
Es hora de agregar una forma de consumir los eventos, lo que será el sistema de eventos. Este sistema contendrá la lógica para lo que deba ocurrir cuando se recibe un evento en específico.

Discutamos cómo manejaremos cada evento:
* Event::PlayerHitObstacle -> aquí es donde irá la reproducción del sonido, pero volveremos a ello más tarde
* Event::EntityMoved(EntityMoved { id }) -> aquí agregaremos la lógica para validar si la entidad que se movió es una caja y si está en una meta o no
* Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) -> este es donde se reproducirá el sonido, pero regresaremos a esto más tarde

```rust
// event_system.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:1:34}}
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:36:63}}
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:71:78}}

```

## Recursos de audio
Ahora que tenemos los eventos en su lugar, agreguemos los recursos de audio. He seleccionado 3 sonidos de este [paquete de recursos](https://opengameart.org/content/512-sound-effects-8-bit-style), pero puedes seleccionar los que gustes.

Sonido correcto [aquí](./sounds/correct.wav)

Sonido incorrecto [aquí](./sounds/incorrect.wav)

Sonido de la pared [aquí](./sounds/wall.wav)

Agreguemos estos sonidos a una nueva carpeta dentro de resources.

```
.
├── resources
│   ├── images
│   │   ├── box_blue_1.png
│   │   ├── box_blue_2.png
│   │   ├── box_red_1.png
│   │   ├── box_red_2.png
│   │   ├── box_spot_blue.png
│   │   ├── box_spot_red.png
│   │   ├── floor.png
│   │   ├── player_1.png
│   │   ├── player_2.png
│   │   ├── player_3.png
│   │   └── wall.png
│   └── sounds
│       ├── correct.wav
│       ├── incorrect.wav
│       └── wall.wav
├── Cargo.lock
└── Cargo.toml
```

## Depósito de audio
Para reproducir el sonido los archivos wav deben ser cargados. Para evitar cargarlos al vuelo cada vez que se desee reproducirlos crearemos un depósito de audio y los cargaremos al inicio del juego.

Utilizaremos un recurso para almacenar el audio.

```rust
// audio.rs
{{#include ../../../code/rust-sokoban-c03-03/src/audio.rs:6:9}}
```

Y registremos este recurso como siempre.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-03/src/resources.rs:14:20}}
```

Y agreguemos el código para inicializar el depósito.

```rust
// audio.rs
{{#include ../../../code/rust-sokoban-c03-03/src/audio.rs:21:32}}
```

## Reproduciendo audio
Finalmente, agreguemos la habilidad para reproducir el sonido que se encuentra en el depósito.

```rust
// audio.rs
{{#include ../../../code/rust-sokoban-c03-03/src/audio.rs:11:19}}
```

Y ahora reproduzcamos el sonido desde el sistema de eventos.

```rust
    // event_system.rs
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:24:37}}
                        // ...
{{#include ../../../code/rust-sokoban-c03-03/src/systems/event_system.rs:61:73}}
```

¡Ahora ejecuta el juego y disfruta esos efectos de sonido!


<video width="75%" controls>
    <source src="./videos/audio.mov" type="video/mp4">
</video>

> **_CODELINK:_**  Puedes ver el código completo de este ejemplo [aquí](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-03).
