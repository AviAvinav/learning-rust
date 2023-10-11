fn main() {
    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter > 11 {
    //         break counter * 2;
    //     }
    // };
    // adding a statement after `break` like on line 8 returns the value of the expression after
    // breaking the loop

    // println!("The result is {result}")

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //
    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut number = 3;
    //
    // while number != 0 {
    //     println!("{number}!");
    //
    //     number -= 1;
    // }
    //
    // println!("LIFTOFF!!!");

    // while loop using if-else-loop

    // fn wheelee(condition: bool, pstr: &str) {
    //     let mut i = 0;
    //     loop {
    //         if condition && i < 5 {
    //             println!("{pstr}");
    //             i += 1
    //         } else {
    //             break;
    //         }
    //     }
    // }
    //
    // wheelee(true, "hello")

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    //
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //
    //     index += 1;
    // }

    for number in 1..4 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
