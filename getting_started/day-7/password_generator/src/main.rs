use rand::Rng;

fn main() {
    let password = generate_password(); 

    println!("{}", password);
}

fn generate_password() -> String {
    // create a new password string
    let mut password = String::new();
    
    // iterate 64 times (length of password)
    for _ in 0..64{
        // create a number from 33 - 126 (ASCII Range)
        let number: u8 = rand::thread_rng().gen_range(33..=126);

        // convert number into a char and push it to the string
        let character: char = char::from(number);

        password.push(character);
    }

    password
}

