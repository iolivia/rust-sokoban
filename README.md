# Rust sokoban

## :dart: What is it?
This repository hosts the Rust Sokoban book and source code used in the book. It's using [mdbook](https://github.com/rust-lang/mdBook).

![Demo](src/images/movement.gif)

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
$ cd code/rust-sokoban-01
$ cargo run --release
```

## :muscle: Contribute

### Add a new section
1. Copy the latest `code/rust-sokoban-x` folder to  `code/rust-sokoban-x+1`
1. Add the code changes of the topic you want to illustrate, try to keep each section self contained and relatively simple to grasp
1. Make sure the code compiles (ideally without warnings)
1. Add a new entry in `src/SUMMARY.md` - each md file should be in the format `cxx-yy-text.md`
1. Fill the markdown in and use code references with line numbers pointing to the `code/rust-sokoban-x+1` folder
1. Create a PR and wait for the :clap: :tada:

### Create an issue
Have a look in the issues list of any duplicates, if you can't find anything create a new one!

## :car: License
MIT
