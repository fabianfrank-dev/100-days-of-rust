struct User{
    name: String,
    age:i32,
    height: i32,
}

impl User{
    fn simple_string(&self) -> String{
        format!("Users name is {}, I am {} years old and {} cm tall", self.name, self.age, self.height)
    }

    fn grow(&mut self, h: i32){
        self.height += h;
    }

    fn die(self){
        println!("Died: {}", self.simple_string());
    }
}

fn main() {
    let mut user = User{
        name: "Person".to_string(),
        age: 43,
        height: 192
    };

    println!("{}", user.simple_string());
    user.grow(-12);
    println!("{}", user.simple_string());

    user.die();
}
