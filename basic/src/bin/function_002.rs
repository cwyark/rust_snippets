// Create a function named increment that takes a mutable integer reference (i.e., &mut i32) and increases its value by 1

fn increment(i: &mut u32) {
    *i += 1;
}

fn main() {
    let mut a: u32 = 1000;
    increment(&mut a);
    println!("a is {a}");
}
