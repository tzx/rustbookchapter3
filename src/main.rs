use std::io;

fn main() {
    let mut what_to_do = String::new();
    println!("Choose what you do to do (1, 2, 3): ");
    println!("1: Convert Temperature");
    println!("2: nth Fibonacci Number");
    println!("3: Print Twelve Days of Christmas");

    io::stdin().read_line(&mut what_to_do)
        .expect("Fail to read line!");
    let what_to_do : u32 = what_to_do.trim().parse()
        .expect("Not a number!");
    println!("You selected #{}.", what_to_do);

    match what_to_do {
        1 => println!("temp func"),
        2 => println!("fib func"),
        3 => println!("song func"),
        _ => println!("Select a valid number!"),
    }
}
