use std::io::{self, Write};

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
        1 => temp_converter(),
        2 => println!("fib func"),
        3 => println!("song func"),
        _ => println!("Select a valid number!"),
    }
}

fn temp_converter() {
    let mut which_conversion = String::new();
    println!("Choose which conversion (1, 2):");
    println!("1: F -> C");
    println!("2: C -> F");

    io::stdin().read_line(&mut which_conversion)
        .expect("Fail to read line!");
    let which_conversion : u32 = which_conversion.trim().parse()
        .expect("Not a number!");

    if which_conversion == 1 || which_conversion == 2 {
        let mut temp = String::new();
        print!("Type your temperature to convert: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut temp)
            .expect("Fail to read line!");
        let temp:f64 = temp.trim().parse()
            .expect("Not a number!");

        if which_conversion == 1 {
            let conversion = (temp - 32.0) * (5.0/9.0);
            println!("{:.3}F is {:.3}C", temp, conversion);
        } else {
            let conversion = (temp * (9.0/5.0)) + 32.0;
            println!("{:.3}C is {:.3}F", temp, conversion);
        }
    } else {
        println!("Not a valid selection!");
    }
}
