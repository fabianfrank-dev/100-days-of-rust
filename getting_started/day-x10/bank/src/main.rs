pub struct BankAccount{
    owner: String,
    number: String,
    balance: f64,
    frozen: bool
}

impl BankAccount{
    fn new(owner: &str, number: &str) -> Self{
        BankAccount{
            owner: owner.to_string(),
            number: number.to_string(),
            balance: 0.0,
            frozen: false
        }
    }
    fn withdraw(&mut self, amount: f64){
            if amount > self.balance || self.frozen == true{
                println!("Transaction failed due to lack of funds or because your account has been frozen.");
                self.frozen = true;
            }else{
                self.balance -= amount; 
                println!("Success!");
            }
    }
    fn deposit(&mut self, amount: f64){
        self.balance += amount;       
    }
    fn summary(&self) -> String{
        format!("The account {} belongs to {} and contains {} €. Frozen: {}", self.number, self.owner, self.balance, self.frozen)
    }
    fn freeze(&mut self){
        self.frozen = true;
    }
    fn unfreeze(&mut self){
        self.frozen = false;
    }
}
fn main() {
    let mut account = BankAccount::new(
        "Person",
        "123456");

    account.withdraw(100.0);
    account.deposit(500.0);

    println!("{}", account.summary());

}
