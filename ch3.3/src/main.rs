fn main() {
    another_func(32);
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    let x = five();
    println!("{x}");
    println!("{:?}", plus_one(5));
}

fn five() -> i32 {
    5
    // perfectly valid rust function
}

fn plus_one(x: i32) -> i32 {
    x + 1 // putting a semicolon to the end of this will turn it into a statement and with throw an
          // error because the function isn't returning anything now
}

fn empty() -> () {
    // this function returns nothing, `()` is the unit type.
    let _ = 1 + 3; // but it can take statements
}

fn another_func(x: i32) {
    println!("Another function.");
    println!("The value of x is {x}")
}
