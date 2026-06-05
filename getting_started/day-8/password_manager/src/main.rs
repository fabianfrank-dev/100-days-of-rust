use rand::Rng;
use base64::{engine::general_purpose::STANDARD, Engine as _};

struct User{
    username: String,
    password: String
}
fn main() {
    let password = generate_password(64);
    println!("Encoded: {:?}", password);
    
    let password = STANDARD.decode(password.as_bytes());
    println!("Decoded: {:?}", password);

}

fn generate_password(l: u32) -> String{
    let mut password: String = String::new();

    for _ in 0..l{
        let number: u8 = rand::thread_rng().gen_range(33..=126);

        password.push(char::from(number));
    }
    
    let password = STANDARD.encode(password.as_bytes());
    password
    

}


