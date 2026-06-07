use crate::garden::vegetables::Carrot;
use crate::garden::vegetables::Cucumber;

pub mod garden;

fn main() {
    let plant = Carrot{};
    println!("Look, i've got a {:?} in my garden", plant);

    let other_plant = Cucumber{};
    println!("I've also got a {:?}", other_plant);
}

