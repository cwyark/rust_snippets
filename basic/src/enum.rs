#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
enum Speed {
    Medium = 20,
    Slow = 10,
    Fast = 30,
}

struct Velocity {
    direction: Direction,
    speed: Speed,
}

impl Velocity {
    fn call_func(&self) {
        println!("calc_vel");
    }
}

fn main() {
    let velocity = Velocity {
        direction: Direction::Up,
        speed: Speed::Medium,
    };
    println!("direction is {:?}", velocity.direction);
    println!("speed is {:?}", velocity.speed);
    velocity.call_func();
}
