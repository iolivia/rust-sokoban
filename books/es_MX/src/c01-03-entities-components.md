# Componentes y entidades
En esta sección crearemos nuestros componentes, veremos cómo crear entidades y registrar todo para mantener specs contento.

## Definiendo componentes
Empecemos definiendo los componentes. Anteriormente comentamos sobre Posición, Renderizable y Movimiento - haremos Movimiento a un lado por ahora. También necesitaremos algunos componentes para identificar cada entidad - por ejemplo necesitamos un componente Pared para poder identificar una entidad como una pared por el hecho de que tenga asociado un componente de tipo pared.

Con un poco de suerte esto será sencillo, el componente de posición almacena las coordenadas x, y y z que nos indicarán dónde se encuentra algún elemento en el mapa, y el componente renderizable recibirá una ruta que apunte a la imagen que deseamos renderizar. Todos los demás componentes son [componentes de marcado](https://specs.amethyst.rs/docs/tutorials/11_advanced_component.html?highlight=marker#marker-components), sin datos (aún).


```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs:13:42}}
```

Entre el código Rust ya familiar también tenemos nueva sintaxis, estamos utilizando una característica poderosa de Rust llamada `Macros de procedimiento` que se utiliza en `#[storage(VecStorage)]`. Este tipo de macros son en esencia funciones que en tiempo de compilación consumen cierta sintaxis y producen una nueva.

> **_MORE:_**  Lee más sobre las macros de procedimiento [aquí](https://doc.rust-lang.org/book/ch19-06-macros.html).

## Registrando componentes
Para que specs pueda funcionar tenemos que indicarle con anticipación qué componentes estaremos utilizando. Crearemos una función para regsitrar los componentes con specs.

```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs:61:69}}
```

## Creando entidades
Una entidad es simplemente un identificador numérico atado a un conjunto de componentes. La forma como crearemos las entidades es especificando qué componentes contienen.

Así es como ahora luce la creación de entidades.

```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs:71:124}}
```

## Recursos

Puedes haber notado en el código que estamos haciendo referencia a los elementos gráficos que utilizaremos en la creación de entidades. Siéntete libre de crear tus propios gráficos o descargar los que estoy utilizando y que puedes encontrar justo a continuación (simplemente haz clic derecho y guardar imagen como...).

![Piso](./images/floor.png)
![Pared](./images/wall.png)
![Jugador](./images/player.png)
![Caja](./images/box.png)
![Meta](./images/box_spot.png)

Agreguemos las imágenes a nuestro proyecto. Agregaremos una carpeta `resources` en la que tendremos todos nuestros recursos, por ahora solamente serán imágenes pero en el futuro tendremos otro tipo de recursos, como archivos de configuración y/o archivos de audio (más adelante aprenderás todo sobre la reproducción de audio en [Capítulo 3.3 - Sonidos y eventos](/c03-03-sounds-events.html)). También agregaremos una carpeta `images` y en ella colocaremos nuestros archivos png, debería verse como lo que se ve a continuación. Puedes utilizar una estructura diferente de carpetas si así lo deseas, solo asegúrate de utilizar las rutas correctas más adelante en esta sección cuando hagamos uso de las imágenes.

```
├── resources
│   └── images
│       ├── box.png
│       ├── box_spot.png
│       ├── floor.png
│       ├── player.png
│       └── wall.png
├── src
│   └── main.rs
└── Cargo.toml
```

## Creación del mundo
Finalmente, hora de unir todo. Crearemos un objeto specs::World, lo agregaremos a nuestra estructura Game y será la primera cosa que inicializaremos en nuestro main. A continuación tienes el código completo, al ejecutarlo debería renderizarse la misma ventana en blanco que ya vimos anteriormente, ¡pero hemos hecho un progreso tremendo en ya haber configurado los componentes y entidades de nuestro juego! A continuación, nos ocuparemos del renderizado así que ¡finalmente veremos algo en pantalla!

```rust
{{#include ../../../code/rust-sokoban-c01-03/src/main.rs}}
```

Nota que al correr el código se reportarán algunas advertencias en la consola sobre importaciones y/o atributos no utilizados, no te preocupes ya que los corregiremos en los siguientes capítulos.

> **_CODELINK:_**  Puedes ver el código completo en este ejemplo [aquí](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c01-03).
