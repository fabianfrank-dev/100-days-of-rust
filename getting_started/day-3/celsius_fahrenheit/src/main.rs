use std::io;

fn main() {
    startup_event();

    let input = get_values(); 
    let decision: i32 = input.0.trim().parse().unwrap();
    let temp: f64 = input.1.trim().parse().unwrap();
    let result: f64;

    match decision{
        1 => {
            result = c_to_f(temp);
        }
        2 => {
            result = f_to_c(temp);
        }
        _ => {
            result = 0.0
        }
    }

    println!("{}", result);
}

fn startup_event(){
    println!("Welcome to the Celsius to Fahrenheit converter");
    println!("CTRL + C to quit");
}

fn get_values() -> (String, String){
    println!("Enter which unit to convert");
    println!(" 1 - C to F");
    println!(" 2 - F to C");
    println!(" 3 - Quit");

    let mut decision: String = String::new();

    io::stdin()
        .read_line(&mut decision)
        .expect("Failed to read line");
    
    println!("How many degrees?");
    let mut temperature: String = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    (decision, temperature)
}


fn f_to_c(f: f64) -> f64{
    let c: f64 = (f - 32.0) * 0.5556;

    c
}

fn c_to_f(c: f64) -> f64{
    let f: f64 = (c * 1.8) + 32.0;

    f

}
