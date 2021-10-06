# Sistema de renderizado

Es hora de nuestro primer sistema, el sistema de renderizado. Este sistema será responsable de dibujar todas nuestras entidades en pantalla.

## Configuración del sistema de renderizado
En primer lugar vamos a definir la estructura `RenderingSystem`, la cual necesita acceso al contexto ggez para poder renderizar.

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:47:49}}
```

Aquí tenemos alguna sintaxis nueva; `'a` es lo que llamamos una anotación explícita de por vida. Se necesita ya que el compilador no puede ver por cuánto tiempo es válida la referencia en `RenderingSystem`, por lo que tenemos que especificar la anotación de por vida.

> **_MORE:_**  Lee más sobre los tiempos de vida [aquí](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).

Ahora implementemos el trait System para nuestro sistema de renderizado. Aún no se requiere haga acción alguna, solo estamos preparando la estructura. La definición de SystemData significa que tenemos acceso al almacenamiento de los componentes de posición y renderizables, y el hecho de que es un almacenamiento de lectura indica que solo tenemos acceso inmutable, que es exactamente lo que necesitamos.

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:51:57}}
        // implementation here
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:83:84}}
```

Finalmente ejecutemos el sistema de renderizado en nuestro ciclo de dibujado. Esto significa que cada vez que el juego se actualice renderizaremos el estado más reciente de todas nuestras entidades.

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:97:111}}
```

En este momento el juego debería compilar, pero probablemente no hará nada todavía, ya que no hemos implementado el sistema de renderizado ni tampoco creado ninguna entidad.

## Implementación del sistema de renderizado

A continuación tenemos la implementación del sistema de renderizado. Se encarga de algunas cosas:
* limpiar la pantalla (asegurándose de que no mantenemos nada del estado renderizado en el cuadro anterior)
* obtener todas las entidades con un componente renderizable y ordenarlas por su componente z (esto es para asegurarnos de que podemos renderizar algunas encima de otras, por ejemplo el jugador debe estar sobre el piso, de otra forma no podríamos verlo)
* itera las entidades ya ordenadas y renderiza cada una de ellas como una imagen
* finalmente, mostrar todo en la pantalla

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:56:83}}
```

## Agregar entidades de prueba

Creemos algunas entidades de prueba para asegurarnos de que todo funciona correctamente.

```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs:179:204}}
```

Finalmente, unamos todo y pongámoslo a correr. ¡Deberías ver algo como esto! Es súper emocionante, ahora tenemos un sistema de renderizado adecuado y podemos ver algo en la pantalla por primera vez. A continuación, vamos a trabajar en la jugabilidad ¡para que realmente se sienta como un juego!

![Screenshot](./images/rendering.png)

El código final se encuentra a continuación.

> **_NOTE:_**  Nota que esta es una implementación muy básica de renderizado y que conforme el número de entidades crezca el desempeño no será lo suficientemente bueno. Una implementación de renderizado más avanzada que utiliza renderizado por lotes puede verse en [Capítulo 3 - Renderizado por lotes](/c03-04-batch-rendering.html).


```rust
{{#include ../../../code/rust-sokoban-c01-04/src/main.rs}}
```

> **_CODELINK:_**  ¡Puedes ver el código completo de este ejemplo [aquí!](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c01-04).