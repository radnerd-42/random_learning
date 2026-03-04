use std::io; //enable i/o

use rand::Rng; //use Range from the rand library

use std::cmp::Ordering; //For comparisons

fn main() {

    println!("Welcome to the Number Guessing Game!!!");

    loop { //loop to keep playing if you want

        game(); //call the game function

        println!("Play again? (y/n)?");

        let mut choice = String::new(); //user response

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");
        
        let choice = choice.trim().to_lowercase(); //trim and make lowercase

        if choice != "y" {
            println!("See you later!");
            break; //exit
        }

    }
}

fn game() {

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate our number to guess between 1 and 100

    loop {
        println!("Please give me a guess between 1 and 100: ");

        let mut guess = String::new(); //declare the variable guess

        //get the user input and assign it to guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        //make sure that guess is a number, if not just ask again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        
        //compare guess with the secret number, and determine where the player is
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!!!");
                break; //End the game if they guess right
            }
        }
    }
}
