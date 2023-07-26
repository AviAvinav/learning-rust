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

Cargo is the compiler and package manager for Rust. Always use it in a real project.

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

**NOTE**: Variables are always scoped, use [`const`](#Constants) to define globally.

## String Interpolation

Basically inserting a variable or smh into a string, what we do in javascript, with `${}` inside strings. In Rust, we do it with just `{}`

## Dependencies

You can install dependencies by updating `cargo.toml` with the dep name and version. The next time you then run `cargo run` or
`cargo build` it will get that dep from crates.io

## `match`

It is a bit similar to `switch` in other languages, it takes a condition compares it with an Enum and runs cases based on that.
Here's an example from the Rust docs:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## `loop`

The `loop` keyword indicates an infinite loop unlike condition based `while` & `for` loops. Here's an example from the Rust docs:

```rust
// Infinite loop
loop {
    count += 1;

    if count == 3 {
        println!("three");

        // Skip the rest of this iteration
        continue;
    }

    println!("{}", count);

    if count == 5 {
        println!("OK, that's enough");

        // Exit this loop
        break;
    }
}
```

## Constants

Constants in Rust are sort of like variables, since, here variables are immutable by default, but while variables can be made mutable
with the `mut` keyword, constant are forever immutable. Also, you need to give it a type on initiation unlike variables where it isn't
necessary. Constants can either be scoped or global.

**NOTE**: The naming convention in Rust for constants is to use all uppercase with underscores between words

Example:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## Shadowing

In Rust, you can redeclare a variable, after it has been initially declared, the redeclared value is considered for evaluation by the
compiler. This is called shadowing, as in the new declaration overshadows the inital one (only after the second declaration). For example:

```rust
let x = 5;

println!("The value of x (initial) is: {x}");

let x = x + 1;

println!("The value of x (shadowed) is: {x}");
```

The output would be like this:

```
The value of x (initial) is: 5
The value of x (shadowed) is 6
```

**NOTE**: Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this
variable without using the `let` keyword. By using let, we can perform a few transformations on a value but have the variable be immutable
after those transformations have been completed. (From Rust docs)

With shadowing you can also change a variable's type you cannot do that with mutability.

## Scalar Data Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

## Integer Types

An integer type is as it says an integer, no decimals or fractions. In Rust, and integer type can either be signed (start with `i`) or unsigned
(start with `u`). If you want a 32 bit integer you can assign a type `u32` or `i32`, similarly for 8, 16, 64, & 128 bit integers. Additionally,
the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64
bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture. The primary situation in which you’d use isize or usize is when indexing some sort of collection.

But what does signed and unsigned mean? Signed means that the integer can have a `+` or a `-` sign, while an unsigned integer will always be positive,
that is, always a `+`.

Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

Same as javascript, number literals can also use \_ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.

## Floating Point Types

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

There are two ways I figured to convert floating-point numbers to integers:

```rust
let x = 2.9;
let y: f32 = 3.2;

let ftoi = x as i32; // "ftoi" becomes 2
let ftoi = y.trunc(); // "ftoi2" becomes 3

```

## Numeric Operations

All other operations are normal, just one thing, integer division truncates toward zero to the nearest integer, for example:

```rust
let truncated = -5 / 3; // Results in -1
```

## Boolean Types

Basic. `true`, `false` (remember not capitalized `T` or `F`). It is represented by `bool`, for example: `let f: bool = false;`

## Character Type

Rust’s char type is the language’s most primitive alphabetic type. Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.

**NOTE**: we specify `char` literals with single quotes, as opposed to string literals, which use double quotes.

## Tuples

Tuples in Rust are immutable, we use `()` for a tuple. They can also be destructured like this:

```rust
let tup: (u16, f64, u8) = (500, 6.4, 1); // not necessary to specify the types of tuples explicitly obviously

let (x, y, z) = tup;
```

We can also access a tuple element directly by using a period `.` followed by the index of the value we want to access. For example:

```rust
let tup: (i16, f64, u8) = (500, 6.4, 1);

let five_hundred = tup.0;
```

The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

## Arrays

Unlike a tuple, all elements in an array must have the same type. Arrays in Rust have a fixed length.

It can be declared with explicit typing like this:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5]; // again even tho the number of elements is fixed not necessary to specify types explicitly
```

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```rust
let a = [3; 5];
```

You can access array elements normally like `a[0]`.

If you try to access an element outside the scope of the array, that is for example if the array has 5 elements but you want `a[10]` it will
throw a runtime error. This is an example of Rust’s memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing
