# Módulos

El archivo main se está volviendo bastante grande y como puedes imaginar, esto no será muy sostenible conforme continúe creciendo nuestro proyecto. Afortunadamente, Rust cuenta con el concepto de módulos que nos permitirá dividir muy bien la funcionalidad en archivos separados.

Por ahora, apuntemos a la siguiente estructura de carpetas. Conforme tengamos más componentes y sistemas, probablemente querremos más de un solo archivo, pero este debe ser un buen punto de partida.

```
├── resources
│   └── images
│       ├── box.png
│       ├── box_spot.png
│       ├── floor.png
│       ├── player.png
│       └── wall.png
├── src
│   ├── systems
│   │   ├── input_system.rs
│   │   ├── mod.rs
│   │   └── rendering_system.rs
│   ├── components.rs
│   ├── constants.rs
│   ├── entities.rs
│   ├── main.rs
│   ├── map.rs
│   └── resources.rs
└── Cargo.toml
```

> **_MORE:_**  Lee más sobre los módulos y la administración de proyectos en crecimiento [aquí](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html).

Empecemos por mover todos los componentes a un archivo. No debería haber más cambios además de hacer que algunos campos sean públicos. La razón por la que necesitamos hacer públicos los campos es porque cuando todo estaba en el mismo archivo todos los elementos tenían acceso a todos los demás, lo que fue conveniente para empezar, pero ahora que queremos separar las cosas necesitamos poner más atención a la visibilidad. Por ahora haremos los campos públicos para que todo funcione nuevamente, pero hay una mejor forma que discutiremos más adelante en esta sección. También hemos movido el registro de los componentes a la parte inferior de este archivo lo cual es bastante práctico para cuando agreguemos componentes ya que solo necesitamos modificar este archivo.

```rust
// components.rs
{{#include ../../../code/rust-sokoban-c02-04/src/components.rs:}}
```

Ahora para los recursos.

```rust
// resources.rs
{{#include ../../../code/rust-sokoban-c02-04/src/resources.rs:}}
```

Siguiente, movamos las constantes a su propio archivo. Por ahora estamos colocando directo en código "duro" las dimensiones del mapa, las necesitamos para que en el movimiento sepamos cuándo hemos alcanzado la orilla del mapa, pero como mejora podríamos más tarde almacenar las dimensiones del mapa y hacerlas dinámicas como parte de la carga del mapa.

```rust
// constants.rs
{{#include ../../../code/rust-sokoban-c02-04/src/constants.rs}}
```

Siguiente, el código de creación de entidades ahora se encuentra en el archivo entities.

```rust
// entities.rs
{{#include ../../../code/rust-sokoban-c02-04/src/entities.rs}}
```

Ahora para la carga del mapa.

```rust
// map.rs
{{#include ../../../code/rust-sokoban-c02-04/src/map.rs}}
```

Finalmente, moveremos el código de los sistemas a sus propios archivos (RenderingSytem a rendering_system.rs e InputSystem a input_system.rs). Debiera ser un copiar y pegar del archivo main con algunas importaciones removidas, así que sigue adelante y hazlo.

Ahora la cuestión interesante sobre los sistemas es que es una carpeta con varios archivos en su interior. Si no hacemos nada más e intentamos hacer uso de `RenderingSystem` o `InputSystem` en el archivo main tendremos algunos errores de compilación. Tendremos que agregar un archivo `mod.rs` en la carpeta `systems` e indicar a Rust que queremos exportarlo fuera de esta carpeta. Todo lo que este código está haciendo es indicarle a Rust que queremos que el mundo exterior (el mundo fuera de esta carpeta) pueda acceder los tipos RenderingSystem e InputSystem.


```rust
// systems/mod.rs
{{#include ../../../code/rust-sokoban-c02-04/src/systems/mod.rs}}
```

Asombroso, habiendo hecho lo anterior aquí tenemos cómo debería verse nuestro archivo main simplificado. Nota las declaraciones mod y use después de las importaciones, están indicándole a Rust que queremos hacer uso de esos módulos.

```rust
// main.rs
{{#include ../../../code/rust-sokoban-c02-04/src/main.rs}}
```

Siéntete libre de ejecutar el programa hasta este punto, todo debería funcionar de la misma forma que antes, la única diferencia es que ahora nuestro código es mucho más bonito y está listo para que agreguemos más características asombrosas de Sokoban.

> **_CODELINK:_**  Puedes ver el código completo de este ejemplo [aquí](https://github.com/iolivia/rust-sokoban/tree/master/code/rust-sokoban-c02-04).


