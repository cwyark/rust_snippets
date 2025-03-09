fn main() {
    // a is stored in stack
    let a = 10;
    // b is a pointer where stored at stack, but it points to a memory location where is at
    // heap.
    let b = String::from("hello");
    println!("a is {a}");
    println!("b is {b}");
}
