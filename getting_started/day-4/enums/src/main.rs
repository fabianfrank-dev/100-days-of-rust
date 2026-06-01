#[derive(Debug)]
pub struct Bed{
    size: u32, 
    count: u32,
}

impl Bed{
    fn simple_string(&self) -> String{
        format!("The size of the bedroom is {} m2 and the count is {}", self.size, self.count)
    }
}

#[derive(Debug)]
pub enum Room{
    Kitchen(i32),
    Bedroom(Bed),
    Lounge
}

fn main() {
    use self::Room::*;

    let bed = Bed{
        size: 14,
        count: 2
    };
    let current = Bedroom(bed);

    println!("Hello from the {:?}", current);

    match current {
        Kitchen(n) => println!("The room is a kitchen with {} items", n),
        Bedroom(bed) => println!("{}", bed.simple_string()),
        _ => println!("The room is a lounge")
    }
}
