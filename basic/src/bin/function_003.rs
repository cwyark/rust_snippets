// Implement a function called apply which takes a closure and an integer, then applies the closure to the integer.
// Demonstrate the usage of this function by passing a closure that doubles the number.

fn apply<F>(f: F, n: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(n)
}

fn main() {
    let result = apply(|x| x * 2, 5);
    println!("result: {}", result);
}
