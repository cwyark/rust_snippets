#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
enum Speed {
    Slow = 10,
    Medium = 20,
    Fast = 30,
}

struct Velocity {
    direction: Direction,
    speed: Speed,
}

fn main() {
    let velocity = Velocity {
        direction: Direction::Up,
        speed: Speed::Medium,
    };
    println!("direction is {:?}", velocity.direction);
    println!("speed is {:?}", velocity.speed);
}
