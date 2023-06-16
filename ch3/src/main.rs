fn main() {
    // Ch3.1 Variables Parts
    // let mut x = 5;
    // // Using the `mut` keyword we can make the variable mutable. Variables in Rust are by default
    // // immutable.
    // println!("The value of x is: {x}");
    // // x = 6; 
    // // Here an error happens because we had the variable set as immutable earlier and we were trying to change it.
    // 
    // x = 6;
    // println!("The value of x is: {x}");
    

    let x = 5;

    println!("The value of x (initial) is: {x}");

    let x = x + 1;

    {
        // This shadowing below will only work within this scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        {
            let x = 2;
            println!("The value of x in the inner inner scope is: {x}");
        }
    }

    println!("The value of x is: {x}");
}
