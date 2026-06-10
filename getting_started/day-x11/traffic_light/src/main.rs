use std::{thread, time};

#[derive(Debug)]
enum TrafficLight{
    Red,
    Yellow,
    Green
}

fn main() {
    let mut current = TrafficLight::Green;

    loop{
        match current{
            TrafficLight::Green => {
                current = turn_red(current);
            },
            TrafficLight::Red=>{
                current = turn_green(current);
            },
            _ => continue,
        }
    }
    
}

fn sleep(dur: u64){
    let duration = time::Duration::new(dur, 0);
    thread::sleep(duration);
}

fn turn_red(mut current: TrafficLight) -> TrafficLight {
    println!("{:?}", current);
    sleep(15);
    current = TrafficLight::Yellow;
    println!("{:?}", current);
    sleep(5);
    current = TrafficLight::Red;
    println!("{:?}", current);
    sleep(20);

    current
}


fn turn_green(mut current: TrafficLight) -> TrafficLight {

    current = TrafficLight::Yellow;
    println!("{:?}", current);
    sleep(5);
    current = TrafficLight::Green;

    current
}

