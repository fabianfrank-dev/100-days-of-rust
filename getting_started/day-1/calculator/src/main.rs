use std::io;

fn main() {

    loop {
        // start event
        startup_event();

        // initialise user decision
        let decision: String = user_choice();
    
        // create new string variables
        let mut a: String = String::new();
        let mut b: String = String::new();
        
        // get input from user
        println!("Enter first number");
        io::stdin()
            .read_line(&mut a)
            .expect("Failed to read line");

        println!("Enter second number");
        io::stdin()
            .read_line(&mut b)
            .expect("Failed to read line");
        
        // add or subtract based on decision
        if decision == "1"{
            add(a, b);  
        } else if decision == "2"{
            subtract(a, b);
        } 
    }
}

fn startup_event(){
    println!("Welcome to the calculator! What do you want to do today?");
    println!(" 1 - Addition");
    println!(" 2 - Subtraction");
    println!(" Ctrl + C to quit");
}

fn user_choice() -> String{
    // set default decision to be 1, addition
    let mut decision: String = String::new();
    
    // get input from user
    io::stdin()
        .read_line(&mut decision)
        .expect("Line could not be read");
    
    return decision.trim().to_string()
}

fn add(a:  String, b: String){
    // parse the strings into i32 integers
    let num1: i32 = a.trim().parse().unwrap();
    let num2: i32 = b.trim().parse().unwrap();

    // add the two numbers
    let sum: i32 = num1 + num2;
    
    println!("Result: {}", sum);
}

fn subtract(a: String, b: String){
    // oarse strings into integers
    let num1: i32 = a.trim().parse().unwrap();
    let num2: i32 = b.trim().parse().unwrap();
    
    // subtract b from a
    let sum = num1 - num2;
    
    // return
    println!("Result : {}", sum); 
}
