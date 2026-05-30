use std::io;
use rand::Rng;

fn main() {
    // main loop
    loop {
        let random: i32 = startup_event();    
        check_guess(random);
    }
}

fn startup_event() -> i32{
    // create random number and explain game to user
    let random: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Welcome to the guessing game!");
    println!("I'm thinking of a number between 1-100");
    println!("You can quit at any time by pressing CTRL + C");
    println!("Enter your guess");

    random
}

fn get_guess() -> String{
    // initialise the guess
    let mut guess: String = String::new();
    
    // get guess from user
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess

}

fn check_guess(random: i32){
    // initialise the guess counter
    let mut guess_amount: i32 = 0;

    loop {
        // get guess from user and parse into integer
        let guess_str: String = get_guess();
        let guess_i32: i32 = match guess_str.trim().parse(){
            Ok(num) => num, 
            Err(_) => {
                println!("Please enter a valid Value");
                continue;
            }
        };
        
        // increase guess count
        guess_amount += 1;
        
        // check if guess is higher, lower or is the random number
        match guess_i32.cmp(&random){
            std::cmp::Ordering::Greater => println!("The number you chose is higher than mine! Guesses: {}", guess_amount),
            std::cmp::Ordering::Less => println!("The number you chose is lower than mine! Guesses: {}", guess_amount),
            std::cmp::Ordering::Equal => {
                println!("Congratulations! {} was the correct number! Number of guesses: {}", random, guess_amount);
                break;
            }
        }
    }

}

    
