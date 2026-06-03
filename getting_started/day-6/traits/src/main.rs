use std::ops::Add;
use std::ops::Sub;

#[derive(Debug)]
 struct Point{
    x: i32,
    y: i32
}

impl Add for Point{
    // declare a type for output
    type Output = Point;
    fn add(self, other: Point) -> Self::Output{
        // add other to self
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point{
    type Output = Point; 
    // subtract other to self
    fn sub(self, other: Point) -> Self::Output{
    Point{
        x: self.x - other.x,
        y: self.y - other.y,
        }
    }

}
fn main() {
    // initialise points
    let a = Point{x:3, y:5};
    let b = Point{x:30, y:50};
    
    // add them up
    let c = a + b;
    
    // subtract another point from an other one
    let d = Point{x:5, y: 7} - Point{x:2, y: 1};
    
    // return
    println!("{:?}", c);
    println!("{:?}", d);
}
