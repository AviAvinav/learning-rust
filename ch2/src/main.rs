use std::io;
// here, we are importing the io (input/output) module from the standard library, this is an
// in-built library. The standard library also provides a lot of other useful packages


fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    // the let keyword is used to create a variable, in this case, guess. The mut keyword is used
    // to make the variable mutable, i.e. it can be changed. The :: syntax is used to indicate
    // that new is an associated function of the String type. An associated function is
    // implemented on a type, in this case, String, rather than on a particular instance of a String.
    // If you want to allocate a size to the string, you can use the following syntax:
    // let mut guess = String::with_capacity(10);
    
    let _apple = 32; 
    // you can define an immutable like this, the type is inferred by the compiler.

    io::stdin()                           // stdin is for standard input, if we didn't import this
                                          // module at the beginnning we could have done std::io::stdin().
                                         
        .read_line(&mut guess)            // reads a line of input and appends it to the string
                                          // given. it is necessary to add the &mut to mutate
                                          // guess.
                                          
        .expect("Failed to read line");   // similar to try except the except runs if the above
                                          // will fail, program will compile without it to but
                                          // compiler will throw a warning, that's cool!

    println!("You guessed: {guess}");
    // the {} is a placeholder for the variable guess, this is called string interpolation in Rust
    // similar to ${} in javascript.
    // you can also use it like this:
    // println!("You guessed: {}", guess);
}
