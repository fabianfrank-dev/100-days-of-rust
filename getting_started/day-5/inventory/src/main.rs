struct Inventory{
    items: Vec<String>,
}

impl Inventory{
    fn add(&mut self, item: String){
        self.items.push(item);
        println!("Addition was successful");
    }
    fn remove(&mut self, item: String){
        let mut found_index = None;

        for (index, current_item) in self.items.iter().enumerate() {
            if current_item == &item{
                found_index = Some(index);
                break;
            }
        }
         if let Some(index) = found_index{
                self.items.remove(index);
                println!("{} has been successfully removed", item);
            }
    }
    fn print(&self){
        println!("Current items in stock:");
        for i in &self.items{
            println!("{}", i);
        }
    }
}

fn main() {
    let a = Vec::new();
    let mut inventory = Inventory{
        items: a
    };

    Inventory::add(&mut inventory, "Glove".to_string());
    Inventory::print(&inventory);
    Inventory::add(&mut inventory, "Shoe".to_string());
    Inventory::print(&inventory);

    inventory.remove("Glove".to_string());
    inventory.print()

}
