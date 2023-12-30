fn main() {
    let mut s = String::new();

    std::io::stdin().read_line(&mut s).expect("Bruh");

    let s: &str = match s.trim().parse() {
        Ok(s) => s,
        Err(_) => "",
    };

    let mut smth = String::from("hello");

    smth.push_str(", sup");
    println!("{}", smth);
}
