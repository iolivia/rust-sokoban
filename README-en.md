[![Code-samples-compile](https://github.com/iolivia/rust-sokoban/workflows/Code-samples-compile/badge.svg)](https://github.com/iolivia/rust-sokoban/actions?query=workflow%3ACode-samples-compile)
[简体中文](README.md)
# [Rust sokoban](https://sokoban.iolivia.me)

## :dart: What is it?
This repository hosts the Rust Sokoban book and source code used in the book. It's using [mdbook](https://github.com/rust-lang/mdBook). You can read the book at [sokoban.iolivia.me](https://sokoban.iolivia.me). 

<img src="src/images/readme.gif" width="80%">

## :running: How to?

### Host the book locally
If not already done, install mdbook.

```
$ cargo install mdbook
```

Serve the book and navigate to http://localhost:3000/ in your local browser.
```
$ mdbook serve
```

### Run the code locally
```
$ cd code/rust-sokoban-c01-01
$ cargo run --release
```

## :muscle: Contribute

### Add a new section
1. Copy the latest `code/rust-sokoban-x` folder to  `code/rust-sokoban-x+1`
1. Add the code changes of the topic you want to illustrate, try to keep each section self contained and relatively simple to grasp
1. Make sure the code compiles (ideally without warnings)
1. Add a new entry in `src/SUMMARY.md` - each md file should be in the format `cxx-yy-text.md`
1. Fill the markdown in and use code references with line numbers pointing to the `code/rust-sokoban-x+1` folder
1. Add a gif towards the end of the new section to showcase the new feature - grab a screen recording and then convert it to a gif (I use ffmpeg for this - `ffmpeg -i Screen_Recording.mov -r 60 -f gif - > moves.gif`)
1. If you add a directory tree listing, use tree - `tree --dirsfirst -I target`
1. Create a PR and wait for the :clap: :tada:

### Create an issue
Have a look in the issues list of any duplicates, if you can't find anything create a new one!

## :car: License
MIT
