# Animaciones
En esta sección agregaremos animaciones a nuestro juego, empezaremos con algunas animaciones básicas pero siéntete libre de agregar más complejas con las ideas en este tutorial. Agregaremos dos animaciones: hacer que el jugador parpadee y que las cajas se sacudan levemente en su lugar.

## ¿Qué es una animación?
Una animación es sencillamente un conjunto de fotogramas reproducidos en un intervalo de tiempo específico que da la ilusión de movimiento. Piensa en esto como en un video (un video es solo un conjunto de imágenes reproducidas en secuencia), pero a muchos menos cuadros por segundo (o fotogramas por segundo).

Por ejemplo, para hacer que nuestro jugador parpadee tendremos tres fotogramas de animación:
1. nuestro jugador actual con los ojos abiertos
1. el jugador con los ojos levemente cerrados
1. el jugador con los ojos completamente cerrados

Si reproducimos estos tres fotogramas en secuencia notarás que parece que el jugador estuviera parpadeando. Puedes intentarlo abriendo las imágenes y cambiando entre ellas rápidamente en la vista previa de tu visor de imágenes.

Hay algunos detalles con lo anterior:
* los recursos deben crearse tomando en cuenta una velocidad de fotogramas específica - nosotros consideraremos 250 milisegundos, lo que significa que reproduciremos un nuevo cuadro cada 250ms, así que tendremos 4 fotogramas por segundo
* los recursos deben ser consistentes entre si - imagina que tuviéramos dos tipos de jugadores que tuvieran recursos y ojos diferentes, tendríamos que asegurarnos de que cuando creáramos los tres fotogramas mencionados anteriormente fueran consistentes, de otra forma los jugadores parpadearían a diferentes velocidades
* diseñar recursos para muchos fotogramas es mucho trabajo, por lo que trataremos de mantener nuestras animaciones simples y enfocarnos en los fotogramas clave

## ¿Cómo funcionará?
Entonces ¿cómo funcionará todo esto en nuestro juego Sokoban? Tendremos que:
1. Cambiar nuestro componente renderizable para que soporte múltiples fotogramas - también podríamos crear un nuevo componente renderizable que maneje animaciones y mantener el que ya tenemos para renderizables estáticos, pero por ahora se siente un poco más limpio unirlos.
1. Modificar la construcción de la entidad del jugador para que maneje fotogramas múltiples.
1. Mantener registro del tiempo que ha transcurrido en nuestro ciclo de renderizado - comentaremos sobre esto con más detalle así que no te preocupes si no es obvio porqué lo necesitamos.
1. Cambiar el sistema de renderizado tomando en cuenta el número de fotogramas, el tiempo y el fotograma que se supone se debe renderizar en un instante específico.

## Recursos
Agreguemos los nuevos recursos para el jugador, deberían verse como siguen. Nota que creamos una convención para nombrar los fotogramas de forma ordenada, no es estrictamente necesario, pero nos ayudará a seguir fácilmente su secuencia.

![Player 1](./images/player_1.png)
![Player 2](./images/player_2.png)
![Player 3](./images/player_3.png)

```
├── resources
│   └── images
│       ├── box_blue.png
│       ├── box_red.png
│       ├── box_spot_blue.png
│       ├── box_spot_red.png
│       ├── floor.png
│       ├── player_1.png
│       ├── player_2.png
│       ├── player_3.png
│       └── wall.png
```

## Renderable
Actualicemos nuestro componente renderizable para que pueda recibir múltiples fotogramas, en lugar de tener una sola ruta, tendremos una lista de rutas, esto debería ser bastante sencillo.

También agregaremos dos nuevas funciones para constuir los dos tipos de renderizables, ya sea con una o con múltiples rutas. Estas dos funciones son funciones asociadas, porque están asociadas con la estructura `Renderable`, pero son el equivalente a funciones estáticas en otros lenguajes ya que no operan sobre instancias (nota que no reciben `&self` o `&mut self` como su primer argumento, lo que indica que las podemos llamar en el contexto de la estructura y no en una instancia de la estructura). También son similares a las funciones  de fábrica, ya que encapsulan la lógica y validación necesarias antes de construir un objeto.

> **_MORE:_**  Lee más sobre funciones asociadas [aquí](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions).

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:19:32}}
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:48}}
```

A continuación, necesitamos una forma de indicar si un renderizable es animado o estático, lo que usaremos en el sistema de renderizado. Podríamos hacer que la variable miembro paths fuera pública y permitirle al sistema de renderizado obtener su longitud e inferir el tipo de renderizable con base en dicha longitud, pero hay una forma más idiomática. Podemos agregar un enum RenderableKind que indique el tipo de renderizable, y agregar a Renderable un método para obtener dicho tipo, de esta forma encapsulamos la lógica dentro del renderizable, y podemos mantener la variable paths como privada. Puedes colocar la declaración de este nuevo enum en cualquier lugar de components.rs, pero idealmente debería estar a un lado de la declaración de Renderable.

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:14:18}}
```

Ahora agreguemos una función que nos indique el tipo de renderizable con base en la variable interna paths.

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:25:40}}
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:48}}
```

Y finalmente, como la variable paths es privada, necesitamos permitir a los usuarios del renderizable obtener una ruta específica de nuestra lista. Para renderizables estáticos este sería el índice 0 de paths (el único índice) y para renderizables con animación necesitaremos que el sistema de renderizado decida cuál de los fotogramas debería ser renderizado con base en el tiempo transcurrido. La única parte complicada es si se solicita un fotograma con índice mayor a los que tenemos, lo envolveremos utilizando el módulo del índice a obtener con la longitud de paths.

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:25}}

    //...

{{#include ../../../code/rust-sokoban-c03-02/src/components.rs:42:48}}
```

## Creación de entidades
Siguiente, actualicemos la entidad de nuestro jugador para que tome en cuenta múltiples rutas. Nota que ahora utilizamos la función `new_animated` para crear el renderizable.

```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c03-02/src/entities.rs:48:60}}
```

Y actualicemos todo lo demás para que haga uso de la función `new_static` - aquí tenemos cómo hacerlo para la creación de la entidad pared, siéntete libre de continuar y aplicarlo a todas las otras entidades estáticas.

```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c03-02/src/entities.rs:5:14}}
```

## Tiempo
Otro componente que necesitaremos para las animaciones es llevar la cuenta del tiempo. ¿Qué tiene que ver el tiempo y cómo se conecta con los fotogramas por segundo? La idea básica es esta: ggez controla qué tan frecuentemente se llama al sistema de renderizado, y esto depende en los cuadros por segundo que a su vez dependen de qué tanto trabajo estamos realizando en cada iteración del ciclo del juego. Ya que es algo sobre lo que no tenemos control, en el transcurso de un segundo podría llamarse 60, 57 o incluso 30 veces. Esto nos indica que no podemos basar nuestra animación en los cuadros por segundo, en su lugar debemos hacerlo en función del tiempo.

Debido a esto necesitamos llevar cuenta de un tiempo al que llamaremos delta - o cuánto tiempo pasa entre el ciclo anterior y el actual. Y debido a que el tiempo delta es mucho más pequeño que el intervalo de fotogramas de nuestra animación (que decidimos fuera 250 ms), necesitamos mantener el delta acumulativo - o cuánto tiempo ha transcurrido desde el inicio del juego.

> **_MORE:_**  Lee más sobre el tiempo delta, cuadros por segundo y ciclos de juego [aquí](https://medium.com/@dr3wc/understanding-delta-time-b53bf4781a03#:~:text=Delta%20time%20describes%20the%20time,drawn%20and%20the%20current%20frame.&text=If%20you%20read%20my%20article,until%20the%20game%20is%20stopped.), [aquí](https://www.reddit.com/r/pcmasterrace/comments/29qcqr/an_explanation_of_game_loops_fps_and_delta_time/) o [aquí](https://www.youtube.com/watch?v=pctGOMDW-HQ&list=PLlrATfBNZ98dC-V-N3m0Go4deliWHPFwT&index=37) .

Ahora agreguemos un recurso para el tiempo, no sigue nuestro modelo de componentes ya que el tiempo es solo un estado global del que debemos llevar registro.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-02/src/resources.rs:45:48}}
```

Y no olvidemos registrar el nuevo recurso.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c03-02/src/resources.rs:12:16}}
```

Y actualicemos este tiempo en nuestro ciclo principal. Afortunadamente ggez nos provee una función para obtener el delta, lo único que tenemos que hacer es acumularlo.

```rust
// main.rs
{{#include ../../../code/rust-sokoban-c03-02/src/main.rs:24:45}}
```

## Sistema de renderizado
Ahora actualicemos nuestro sistema de renderizado. Obtendremos el tipo del renderizable, si es estático simplemente utilizaremos el primer fotograma, de lo contrario averiguamos qué fotograma utilizar con base en el tiempo delta.

Primero agreguemos una función para encapsular la lógica para obtener la imagen correcta.

```rust
// rendering_system.rs
{{#include ../../../code/rust-sokoban-c03-02/src/systems/rendering_system.rs:17}}
    //...
{{#include ../../../code/rust-sokoban-c03-02/src/systems/rendering_system.rs:34:54}}
```

Y finalmente, usemos la nueva función `get_image` dentro de la función run (también tendremos que agregar time a la definición de `SystemData` y algunas importaciones más, pero eso debería ser todo más o menos).

```rust
// rendering_system.rs
{{#include ../../../code/rust-sokoban-c03-02/src/systems/rendering_system.rs:57:81}}

            //...
            
{{#include ../../../code/rust-sokoban-c03-02/src/systems/rendering_system.rs:88}}

        //...

{{#include ../../../code/rust-sokoban-c03-02/src/systems/rendering_system.rs:97}}
{{#include ../../../code/rust-sokoban-c03-02/src/systems/rendering_system.rs:98}}

```

## Animaciones de las cajas
Ahora que hemos aprendido cómo hacer animaciones, extendámoslo para hacer que también las cajas tengan animación. Todo lo que debemos hacer es agregar nuevos recursos y actualizar la creación de las entidades, y todo debería funcionar. Aquí están los recursos que utilicé, siéntete libre de usarlos o ¡de crear nuevos!

![Box red 1](./images/box_red_1.png)
![Box red 2](./images/box_red_2.png)
![Box blue 1](./images/box_blue_1.png)
![Box blue 2](./images/box_blue_2.png)

## Terminando
Esta fue una sección larga, ¡pero espero que la hayas disfrutado! Aquí tienes cómo debería lucir el juego ahora.

![Sokoban animations](./images/animations.gif)

> **_CODELINK:_**  Puedes ver el código completo de este ejemplo [aquí](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c03-02).








