// Write a function in Rust called add_numbers that takes
// two integer parameters and returns their sum.
fn add_numbers(a: u32, b: u32) -> u64 {
    (a + b).into()
}

fn main() {
    let a = 1234;
    let b = 5678;
    let c = add_numbers(a, b);
    println!("a + b = {c}");
}
