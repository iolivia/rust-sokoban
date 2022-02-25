[![Code-samples-compile](https://github.com/iolivia/rust-sokoban/workflows/Code-samples-compile/badge.svg)](https://github.com/iolivia/rust-sokoban/actions?query=workflow%3ACode-samples-compile)
[简体中文](README-zh_CN.md) [English](README.md)

# [Rust sokoban](https://sokoban.iolivia.me)

## :dart: ¿Qué tenemos aquí?
Este repositorio hospeda el libro Rust Sokoban y el código fuente utilizado en el libro. Utiliza [mdbook](https://github.com/rust-lang/mdBook). Puedes leer el libro en [sokoban.iolivia.me](https://sokoban.iolivia.me/es_mx). 

<img src="books/es_MX/src/images/readme.gif" width="80%">

## :running: ¿Cómo?

### Hospedar el libro de forma local
Si aún no lo haces, instala mdbook.

```
$ cargo install mdbook
```

Sirve el libro y navega a http://localhost:3000 en tu navegador local.
```
$ mdbook serve
```

### Ejecutar el código de forma local
```
$ cd code/rust-sokoban-c01-01
$ cargo run --release
```

## :muscle: Contribuir

### Agregar una nueva sección
1. Copia la versión más reciente de la carpeta `code/rust-sokoban-x` a `code/rust-sokoban-x+1`
1. Agrega los cambios al código del tema que quieres ilustrar, intenta mantener cada sección autocontenida y relativamente sencilla de comprender
1. Asegúrate de que el código compila (idealmente sin advertencias)
1. Agrega una nueva entrada en `src/SUMMARY.md` - cada archivo md debería seguir el formato  `cxx-yy-text.md`
1. Llena el contenido utilizando sintaxis markdown y para las referencias de código utiliza números de línea que apunten a la carpeta `code/rust-sokoban-x+1`
1. Agrega un gif hacia el final de la nueva sección para demostrar la nueva característica - crea una grabación de la pantalla y conviértela a gif (yo utilizo ffmpeg para esto - `ffmpeg -i Screen_Recording.mov -r 60 -f gif - > moves.gif`)
1. Si agregas un listado de árbol de directorios, utiliza tree - `tree --dirsfirst -I target`
1. Crea un pull request y espera por los :clap: :tada:

### Crear una incidencia
Da un vistazo a las incidencias ya existentes para asegurar que tu duda aún no ha sido resuelta, si no la encuentras ¡crea una nueva!

### Ayuda con una traducción
1. Crea una copia de la carpeta books/en_US con el código ISO de la nueva traducción (por ejemplo fr_FR, ve la [lista](http://www.lingoes.net/en/translator/langcode.htm))
1. Indica el idioma en book.toml
1. Traduce SUMMARY.md en primer lugar
1. Traduce cada capítulo/sub-capítulo, intenta no modificar ninguna des las imágenes/sonidos/videos o alguna parte de la estructura del libro
1. Siéntete libre de crear un pull request borrador tan pronto como tengas unas cuantas páginas traducidas, esto dará a conocer tu trabajo a otros colaboradores
1. Cuando la traducción esté lista, notifica al dueño del repositorio, se requiere un cambio en el CI para publicar el libro a un nuevo subdominio (sokoban.iolivia.me/fr_fr en este ejemplo)

## :car: License
MIT
