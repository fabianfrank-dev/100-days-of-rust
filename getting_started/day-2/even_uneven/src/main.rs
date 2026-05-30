use std::io;

fn main() {
    loop{
    startup_event();
    let number: String = get_number();
    check_number(number);
    }
}

fn startup_event(){
    println!("Welcome to the Even or Uneven game");
    println!("How it works: You give me a number and I'll say whether it's even or uneven");
    println!("Would you mind sharing your number with me?");
    println!("Remember: CTRL + C to quit");
}

fn get_number() -> String{
    let mut number: String = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    number
}

fn check_number(number_str: String){
    loop{
        // convert number securely to i32
        let number: i32 = match number_str.trim().parse(){
            Ok(num) => num,
            Err(_) => {
            println!("Please enter a valid number");
            break;
            }
        };
            // check if number is divisible by two
        match number % 2{
            0 => println!("{} is even", number),
            _ => println!("{} is uneven", number),
        }
        break;
    }
}
