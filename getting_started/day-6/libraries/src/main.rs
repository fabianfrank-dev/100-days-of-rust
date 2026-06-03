use rand::Rng;
use std::ops::Add;

#[derive(Debug)]
#[derive(Clone)]
struct Point{
    x: i32,
    y: i32
}

impl Add for Point{
    type Output = Point;
    fn add(self, other: Point) -> Self::Output{
        Point{
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Point{
    fn distance(&self, other: Point) -> f64{
        let d: f64 = ((other.x - self.x).pow(2) + (other.y - self.y).pow(2)).into();  
        let r = d.sqrt();
        r
    }
}
fn main() {
    let x1: i32 = rand::thread_rng().gen_range(1..=100);
    let x2: i32 = rand::thread_rng().gen_range(1..=100);

    let y1: i32 = rand::thread_rng().gen_range(1..=100);
    let y2: i32 = rand::thread_rng().gen_range(1..=100);

    let a: Point = Point{x: x1, y: y1};
    let b: Point = Point{x: x2, y: y2};

    let c: Point = a.clone() + b.clone();
    
    println!("Point a: {:?}", a.clone());
    println!("Point b: {:?}", b.clone());
    println!("Added point: {:?}", c);

    let distance = a.distance(b);
    println!("Distance a->b: {:?}", distance);
}
