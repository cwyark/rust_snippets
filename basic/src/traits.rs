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

fn main() {
    let bike = Bike {
        brand: "bike".to_string(),
    };

    let car = Car {
        brand: "car".to_string(),
    };

    bike.forward();
    bike.backward();

    car.forward();
    car.backward();
}
