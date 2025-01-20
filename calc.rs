use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Please input your first number:");

    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Please type a number!");

    println!("Please input your second number:");

    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Please type a number!");

    println!("Please input the operation (+, -, *, /):");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");
    let operation = operation.trim();

    let result = match operation {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => first_number / second_number,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("The result is: {}", result);
}