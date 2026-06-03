use std::io;

fn main() {
    println!("Temperature converter");
    
    loop {
        println!("Enter 1 to convert from Farenheit to Celsius.\nEnter 2 to convert from Celsius to Farenheit.\nEnter 3 to exit.");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid choice.");
                return;
            }
        };
        //Farenheit to Celsius
        if choice == 1 {
            f_to_c();
        //Celsius to FarenHeit
        } else if choice == 2 {
            c_to_f();
        //Exit
        } else if choice == 3 {
            println!("Goodbye!");
            print!("\x1B[2J\x1B[1;1H"); //Clear the screen
            break;
        } else {
            println!("Invalid choice. Enter a valid choice.");
        }
    }
}
//Get user input of temperature in Farenheit and convert to Celsius
fn f_to_c() {
    println!("Please input your termperature in Farenheit: ");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Enter a valid temperature.");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
             println!("Enter a valid temperature.");
             return;
        }
    };
    let cel = (temp - 32.0) * 5.0 / 9.0;
    println!("\n{:.2}* Farenheit converted to Celsius is {:.2}*.\n\n", temp, cel);
}

//Get user input of temperature in Celsius and convert to Farenheit
fn c_to_f() { 
    println!("Please input your temprature in Celsius: ");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Enter a valid temperature.");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
             println!("Enter a valid temperature.");
             return;
        }
    };
    let far = temp * 9.0 / 5.0 + 32.0;
    println!("\n{:.2}* Farenheit converted to Celsius is {:.2}*.\n\n", temp, far);
}
