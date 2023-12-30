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

## Functions

As already mentioned the `main()` function is the heart of everything, so won't talk anything more about it.

So basic stuff:

- `fn` keyword is used to define a function
- Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words
- Rust doesn't care where you define your functions, you can define after or before the `main()` function and still use it inside of it.
- Function parameters always need to be typed

Example of a function with parameters:

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

**Note**: You cannot do something like this in Rust: `let x = (let y = 6);`, because statements & expressions are different things here. Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value.

However you can do this:

```rust
let y = {
    let x = 3;
    x + 1
};
```

because everything in the curly brackets is considered an expression, so y=4 in this case.

**VERY IMP NOTE**: Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

- you can specify a function's return type like this: `fn five() -> i32`
- `()` is the unit type if you give it to a function that means the function must not return anything.

## What is `{:?}`?

{...} surrounds all formatting directives. : separates the name or ordinal of the thing being formatted (which in this case is omitted, and thus means "the next thing") from the formatting options. The ? is a formatting option that triggers the use of the std::fmt::Debug implementation of the thing being formatted, as opposed to the default Display trait, or one of the other traits (like UpperHex or Octal).

Thus, {:?} formats the "next" value passed to a formatting macro, and supports anything that implements Debug.

More: https://doc.rust-lang.org/std/fmt/index.html

## if-else statements

The `if` & `else` statements work the same as you had expect it to work in any other language, still here's an example:

```rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

One thing to note in this however is that the condition in Rust must always be a boolean value unlike in languages like python and js, Rust doesn't automatically
convert non-boolean types to boolean.

You can also do something like this for multiple cases:

```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

You can also conditionally assing variables using `if` statement:

```rust
let smth = if number>4 { 5 } else { 6 };
// for some variable number
```

However do keep in mind that in the above example, both 5 and 6 were of the same type, i:e, i32 but if one was string and other an integer, this would have led
to an error. So, the values in if-else during assignment to a variable should be of the same type.

## Why rust is faster than python or js?

This is just my opinion from what I have learned until now. but the Rust is fast (one of the many reasons) is that it knows the types of everything in advance,
it doesn't have to wait to determine the types at runtime, where it has to take into account multiple different situations depending on the type determined at runtime.

## Loops

We have discussed a bit about `loop` earlier, more on that now. There are 3 types of loops in Rust: `loop`, `while`, `for`.

"One of the uses of a `loop` is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result
of that operation out of the loop to the rest of your code. To do this, you can add the value you want returned after the break expression you use to stop the loop;
that value will be returned out of the loop so you can use it" - Rust book

You can also use `loop` like above to assign a value to a variable:

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter > 11 {
        break counter * 2;
    }
};

```

### Loop labels

"If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that you can then use with
break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop."

Loop labels begin with a single quote(`'`), here's an example of a loop label from the book:

```rust
let mut count = 0;
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }

    count += 1;
}
println!("End count = {count}");

```

Here the outer loop has the label `'counting_up` and thus `break 'counting_up` breaks the outer loop instead of the inner loop.

### while

There is also the normal `while` loop here's an example just in case:

```rust
let mut number = 3;

while number != 0 {
    println!("{number}!");

    number -= 1;
}

println!("LIFTOFF!!!");
```

and ofc, the condition in this case should also be a boolean definitely.

We can also recreate a `while` loop using `loop`, `if`, `else`, here is a simple recreation:

```rust
let mut number = 3;

while number != 0 {
    println!("{number}!");

    number -= 1;
}

println!("LIFTOFF!!!");
```

**NOTE**: You can't use negative numbers for indexing in Rust, like a[-1] is invalid

### for

Similar to for loops in other languages, no real difference, here's an example:

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}
```

**NOTE**: We can just specify a range like this in Rust:

```rust
for number in 1..4 {
    println!("{number}");
}
```

This will print 1 to 3, we can also use `rev()` to reverse a list, like `for number in (1..4).rev()`.

## Stack & Heap

It is necessary to understand Stack and Heap in order to understand Ownership, a unique feature of Rust.

Stack: The same as the general programming concept of stack, which follows LIFO, you cannot take stuff out of the middle. All data on a stack must have a known, fixed size.

Data with an unknown size at compile time or size that may change must be stored in a heap instead of a stack.

Heap: You can think of a heap as big empty space, where you can allocate data. The memory allocator finds an empty spot in the heap, marks it as being in use and returns a pointer, which is the address of the data on the heap. Since,
pointers have a fixed, known size they can be stored in a stack. Let's take an example, assume you went to a restaurant and the waiter points you to a table suitable for all of you,
if a friend is late and comes later the waiter will point your friend to you. So, the waiter is the pointer is in this case, the restaurant a heap and your group data.

Pusing to a stack is faster than allocating in the heap because in a stack the allocator doesn't need to look for a place, the location is always at the top of the stack.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant
taking orders from many tables. It’s most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process.
By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap).

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. Once you understand ownership,
you won’t need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

## Ownership rules in Rust

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## The String type

The String type is different from the string literal, the string literal is not mutable.

Benefit?

Well using the String type you can append things to a string, because it is not necessary that the string we wish to append to is always fixed, it maybe coming from an input,
in that case we will use the `from` function from the String type like so:

```rust
let mut s = String::from("hello");

s.push_str(", world!");
println!("{}", s);
```

The String type and string literal differ because they deal with memory differently.
