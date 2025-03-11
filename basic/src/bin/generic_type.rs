fn find_largest<T: PartialOrd + Copy>(arr: &[T]) -> T {
    let mut largest = arr[0];
    for &elem in arr.iter() {
        if elem > largest {
            largest = elem;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

#[derive(Debug)]
enum Color<T> {
    black(T),
    red(T),
    blue(T),
}

fn main() {
    let a = find_largest(&vec![0, 2, 4, 5]);
    println!("a is {}", a);
    let b = find_largest(&vec!["a", "b", "c", "d"]);
    println!("b is {}", b);
    let point_a = Point { x: 10, y: 12 };
    let point_b = point_a.get_x();
    println!("get_x is {}", point_b);
    println!("point_a.x is {}", point_a.x);
    let color_black = Color::black(10);
    println!("color black is {:?}", Color::black(10));
}
