# Learning Rust

I am learning the Rust language using [the book](https://doc.rust-lang.org/stable/book) and this contains my notes

## `main()` function

The `main()` function is a special function that is called first in a rust file, basically this
is the heart of your file, this is where everything you want to run should be referenced.

## How to define a function

you can define a function with the syntax:

```
fn func_name (your_arg: arg_type, other_arg: arg_type) {
    // ... your stuff here
}
```

## Comments

comments can be written like this:

```
// ... whatever
```

## Cargo

Cargo is basically the compiler and package manager for Rust. Always use it in a real project.

- a new project can be made with `cargo new project_name`
- you can compile the project with `cargo build`. this will make the executable in `target/debug`
  - for building a release version run `cargo build --release` instead. this will make the executable file in `target/release`
- you can compile and run the executable with `cargo run`
- you can check if the project can be compiled without errors with `cargo check`

## The `!`

you need to put `!` at the end of Rust's in-built methods and functions like `println!`, if you don't it won't consider it as a Rust
in-built method/function but search for function defined `println` in the file.
