use std::io;

fn main() {
    println!("Simple Calculator\nEnsure there spaces between values and signs.\nValid signs are + - * /\nEnter your problem:");

    let mut problem = String::new();
    io::stdin().read_line(&mut problem).expect("Failed to read input.");

    let tokens: Vec<&str> = problem.split_whitespace().collect();

    if tokens.len() < 3 {
        println!("Input appears incomplete. Please provide the full problem.");
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Ensure your first value is a number.");
            return;
        }
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Ensure your second value is a number.");
            return;
        }
    };

    let result = match operator {
        "+" => addition(num1, num2),
        "-" => subtraction(num1, num2),
        "*" => multiplication(num1, num2),
        "/" => division(num1, num2),
        "%" => modulo(num1, num2),
        _ => {
            println!("Make sure that you are using a valid sign. +-*/");
            return;
        }
    };

    println!("{:.2}", result);
    
}

fn addition(a: f64, b: f64) -> f64 {
    a + b
}

fn subtraction(a: f64, b: f64) -> f64 {
    a - b
}

fn multiplication(a: f64, b: f64) -> f64 {
    a * b
}

fn division(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Dividing by 0 is not allowed.");
        std::process::exit(1);
    }
        a / b
}

fn modulo(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Dividing by 0 is not allowed.");
        std::process::exit(1);
    }
        ((a % b) + b) % b
}
