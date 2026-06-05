use std::io;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Products{
    item: String,
    amount: u32
}   

fn view(products: &Vec<Products>){

    // iterate through items to print
    for item in products{
        println!("{}: {}", item.item, item.amount);
    }
    
}
    // 
    
fn add(products: &mut Vec<Products>){
        
    // initialise new input variables
    let mut item: String = String::new();
    let mut amount: String = String::new();
        
    // ask for user input and get it
    println!("Which item would you like to add?");

    io::stdin()
        .read_line(&mut item)
        .expect("Failed to read line");

    println!("Gotcha! How many items of '{}' do you need?", item);
    
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
        
        // check for valid u32 values until it's fulfilled
    loop {
       let u32_am: u32 = match amount.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        println!("Got it! Added {} instances of {} to the list!", u32_am, item);
        // insert product into list
        let product = Products{
            item: item,
            amount: u32_am
        };
        products.push(product);
       break;
       }
    }

fn delete_from(products: &mut Vec<Products>){
    
}

fn print_to_file(products: &Vec<Products>) -> std::io::Result<()>{
    let mut file: File = File::create("shopping_list.txt")?;
    
    for product in products{
        writeln!(&mut file, "{} {}", product.amount, product.item)?;
    }   


    Ok(())
}

fn main() {
    user_decision();
}

fn user_decision(){
    let mut vec = Vec::new();
    
    
    loop {
        println!("Hello there, what would you like to do?");
        println!("1 - Add product to the list");
        println!("2 - View list");
        println!("4 - Print to file");
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
                add(&mut vec); 
            }
            2 =>{
                view(&vec);
            }
            4 =>{
                let _ = print_to_file(&vec);
            }
            _ => {
                println!("Please enter a valid number");
                continue;
            }
        }
    }
}





