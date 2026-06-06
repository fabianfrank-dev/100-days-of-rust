struct Square{
    width: u32,
    height: u32
}

impl Square{
    fn new(w: u32,h: u32) -> Square{
        Square{
            width: w,
            height:h
        }
    }
    fn area(&self) -> u32{
        let area = self.width * self.height;

        area
    }
    fn is_square(&self) -> bool{
        if self.width == self.height{
            return true;
        }

        return false;
    }
    fn perimeter(&self) -> u32{
        let perimeter = 2 * (self.width + self.height);

        perimeter
    }
}
fn main() {
    let square: Square = Square::new(20,12);
    let area = square.area();
    let is_sqr = square.is_square();
    let perimeter  = square.perimeter();
    
    println!("Width: {}, height: {}", square.width, square.height);
    println!("Area: {}", area);
    println!("Is it a square? {}", is_sqr);
    println!("Perimeter: {}", perimeter);

}
