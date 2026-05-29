use std::io;

fn main() {
    startup_event();
    let decision = user_choice();
}

fn startup_event(){
    println!("Welcome to the calculator! What do you want to do today?");
    println!(" 1 - Addition (Default)");
    println!(" 2 - Subtraction");
    println!(" 3 - quit");
}

fn user_choice() -> String{
    // set default decision to be 1, addition
    let mut decision: String = String::new();

    io::stdin()
        .read_line(&mut decision)
        .expect("Line could not be read");
    
    return decision
}

fn add(a:  i32, b: i32) -> i32{
    let sum = a + b;
    
    return sum
}
