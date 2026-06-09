use std::process;

fn main() {
    let result = calculate(5.0, "/", 3.0);
    println!("{}", result);
}

fn calculate(a: f64, op: &str, b: f64) -> f64{
    let mut result;

    match op{
        "+" => {
            result = a + b;
        }
        "-" => {
            result = a - b;
        }
        "*" => {
            result = a * b;
        }
        "/" => {
            match b{
                0.0 => {
                    println!("Division by 0 not possible");
                    process::exit(1)
                }
                _ => {
                    result = a / b;
                }
            }
        }
        _ => {
            println!("Invalid operation");
            process::exit(1);
        }
    }
    
    result
}
