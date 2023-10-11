use std::io::stdin;

fn main() {
    let mut number = String::new();

    stdin().read_line(&mut number).expect("Failed to read line");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let smth = if number > 4 { 5 } else { 6 };
    println!("{smth}")
}
