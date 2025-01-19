struct Bike {
    brand: String,
}

struct Car {
    brand: String,
}

trait Ride {
    fn forward(&self);

    fn backward(&self);
}

impl Ride for Car {
    fn forward(&self) {
        println!("car forward");
    }

    fn backward(&self) {
        println!("car backward");
    }
}

impl Ride for Bike {
    fn forward(&self) {
        println!("bike forward");
    }

    fn backward(&self) {
        println!("bike backward");
    }
}

// argument &dyn Ride means any value that implement the Ride trait
fn move_forward(vehicle: &dyn Ride) {
    vehicle.forward();
}

// argument &dyn Ride means any value that implement the Ride trait
fn move_backward(vehicle: &dyn Ride) {
    vehicle.backward();
}
fn main() {
    let bike = Bike {
        brand: "bike".to_string(),
    };

    let car = Car {
        brand: "car".to_string(),
    };

    move_forward(&bike);
    move_forward(&car);

    move_backward(&bike);
    move_backward(&car);
}
