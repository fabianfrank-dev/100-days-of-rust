use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Products{
    product: HashMap<String, u32>
}   
impl Products{
    fn view(&self){
        // iterate through items to print
        for item in &self.product{
            println!("{:?}", item);
        }
    }
    // 
    fn add(&mut self){
        // initialise new input variables
        let mut item: String = String::new();
        let mut amount: String = String::new();
        
        // ask for user input and get it
        println!("Which item would you like to add?");

        io::stdin()
            .read_line(&mut item)
            .expect("Failed to read line");

        println!("Gotcha! How many items of '{}' do you need?", item.clone());
        
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");
        
        // check for valid u32 values until it's fulfilled
        loop {
            let amount_u32: u32 = match amount.trim().parse(){
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number");
                    continue;
                }
            };
            println!("Got it! Added {} instances of {} to the list!",amount, item.clone());

            // insert product into list
            self.product.insert(item.trim().into(), amount_u32);
            break;
        }

    }
}

fn main() {
    user_decision();
}

fn user_decision(){
    let mut products = Products{product: HashMap::new()};   
    
    loop {
        println!("Hello there, what would you like to do?");
        println!("1 - Add product to the list");
        println!("2 - View list");
        println!("Press CTRL + C");

        let mut decision: String = String::new();
    
        io::stdin()
            .read_line(&mut decision)
            .expect("Failed to read line");

        let decision_u32: u32 = match decision.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        match decision_u32 {
            1 => {
                products.add(); 
            }
            2 =>{
                products.view();
            }
            _ => {
                println!("Please enter a valid number");
                continue;
            }
        }
    }



}





