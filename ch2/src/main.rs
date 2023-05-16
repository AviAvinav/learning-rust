use std::io;
// here, we are importing the io (input/output) module from the standard library, this is an
// in-built library. The standard library also provides a lot of other useful packages
// these packages in Rust are called crates
use rand::{Rng, thread_rng}; // here are Rng is called a trait, covered more in ch10
                             // this is similar to doing import {Stuff} from "stuff" in javascript,
                             // but if you want only one thing no need to use {}
use std::cmp::Ordering;      // Ordering is an enum, covered more later
                             // cmp is a module for comparison


fn main() {
    println!("Guess the number!");

    let secret_num = thread_rng().gen_range(1..=100); // this is inclusive of both 1 and 100

    println!("Please input your guess.");

        let _apple = 32; 
    // you can define an immutable like this, the type is inferred by the compiler.

                                          
    println!("The secret number is: {}", secret_num);
                                           
    loop {                                 // noice, there is no for or while loop bullshit just
                                           // loop.
        
        let mut guess = String::new();

        // the let keyword is used to create a variable, in this case, guess. The mut keyword is used
        // to make the variable mutable, i.e. it can be changed. The :: syntax is used to indicate
        // that new is an associated function of the String type. An associated function is
        // implemented on a type, in this case, String, rather than on a particular instance of a String.
        // If you want to allocate a size to the string, you can use the following syntax:
        // let mut guess = String::with_capacity(10);
    


        io::stdin()                           // stdin is for standard input, if we didn't import this
                                              // module at the beginnning we could have done std::io::stdin().
                                         
            .read_line(&mut guess)            // reads a line of input and appends it to the string
                                              // given. it is necessary to add the &mut to mutate
                                              // guess.
                                          
            .expect("Failed to read line");   // similar to try except the except runs if the above
                                              // will fail, program will compile without it to but
                                              // compiler will throw a warning, that's cool!

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };
        // converts guess from number to an undefined 32-bit
                                           // integer
     

        println!("You guessed: {guess}");
        // the {} is a placeholder for the variable guess, this is called string interpolation in Rust
        // similar to ${} in javascript.
        // you can also use it like this:
        // println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

        // match is similar to switch-case in other languages
}
