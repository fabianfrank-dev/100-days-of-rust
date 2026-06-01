// Objective: Read arguments and say hello to everyone whose name begins with W
use std::env::args;

fn main() {
    for i in args(){
        if let Some(c) = i.chars().next(){
            match c {
                'w'|'W' => println!("Hello, {}", i), 
                _ => {}
            }
        }
    }
}
