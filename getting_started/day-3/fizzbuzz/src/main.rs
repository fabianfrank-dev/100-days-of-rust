use std::io;

fn main() {
    // startup event with instructions
    startup_event();
    
    // get number and check it
    get_check_number();
}    

fn startup_event(){
    println!("Welcome to FIzzbuzz");
    println!("Enter a number and I'll say fizz if it's divisible by 3 or buzz if it's divisible by 5");
    println!(" CTRL + C to quit");
}

fn get_input() -> String{
    // initialise number as string
    let mut number: String = String::new();
    
    // get number from user
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    
    // return
    number
}

fn get_check_number(){
    // loop
    loop{
        // get number from user and convert securely to i32
        let number_str: String = get_input();
        let number_i32: i32 = match number_str.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        
        // initialise mod numbers
        let mod_3: i32 = number_i32 % 3;
        let mod_5: i32 = number_i32 % 5;
        
        // check for divisible by 3 or 5
        match mod_3 == mod_5 && mod_5 == 0{
            true => println!("FizzBuzz"),
            false => {
                match mod_3{
                    0 => println!("Fizz"),
                    _ => {
                        match mod_5{
                            0 => println!("Buzz"),
                            _ => println!("Inconclusive")
                        }
                    }
                }
            }
        }

    }
}
