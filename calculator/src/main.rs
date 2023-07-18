use std::io::{self, Write};

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            a / {
                if b == 0 as f64 {
                    panic!("Second number can not be zero")
                } else {
                    b
                }
            }
        }
    }
}

fn main() {
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_number: f64 = input.trim().parse().unwrap();

    print!("Enter the operation (+, -, *, /): ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let operation = match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => panic!("Invalid operation"),
    };

    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let second_number: f64 = input.trim().parse().unwrap();

    let result = calculate(operation(first_number, second_number));

    println!("Result: {}", result);
}
