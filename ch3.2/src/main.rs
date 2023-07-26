fn main() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.9;
    let y: f32 = 3.2;

    let ftoi = x as i32;
    println!(
        "The original floating type decimal was {}, it has been converted to integer {}",
        x, ftoi
    );

    let ftoi2 = y.trunc();

    println!(
        "The original floating type decimal was {}, it has been converted to integer {}",
        y, ftoi2
    );

    let truncated = 5 / 3;
    let truncated2 = -5 / 3;

    println!(
        "The truncated value of 5/3 is {}, and the truncated value of -5/3 is {}",
        truncated, truncated2
    );

    let yes: bool = true;

    let ch = 'ℤ';

    let tup: (u16, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let first = tup.0;

    println!("The value of y is: {y}");
    println!("The first element of tuple is {first}");
}
