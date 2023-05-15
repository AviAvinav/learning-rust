# Learning Rust

I am learning the Rust language using [the book](https://doc.rust-lang.org/stable/book) and this contains my notes

## `main()` function

The `main()` function is a special function that is called first in a rust file, basically this
is the heart of your file, this is where everything you want to run should be referenced.

## How to define a function

You can define a function with the syntax:

```rust
fn func_name (your_arg: arg_type, other_arg: arg_type) {
    // ... your stuff here
}
```

## Comments

Comments can be written like this:

```rust
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

You need to put `!` at the end of Rust's in-built methods and functions like `println!`, if you don't it won't consider it as a Rust
in-built method/function but search for function defined `println` in the file.

## Imports

You can import a module like with the `use` keyword like `use std::io`

## Variables

`let` keyword is used to define a variable, for example:

```rust
let apples = 32;
```

Rust will automatically infer the type because it isn't stupid like javascript. By default, these variables are immutable but can be
mutated with the `mut` keyword like so:

```rust
let mut guess = String::new();
```

In the above we are not assigning to `guess` but we are declaring it as a string. Of course, you can also declare it with a value like
`let mut guess = "stuff"`

## String Interpolation

Basically inserting a variable or smh into a string, what we do in javascript, with `${}` inside strings. In Rust, we do it with just `{}`
