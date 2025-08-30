/*
- Swap Two Values Using Mutable References
    Implement a function named swap that accepts two mutable references to i32 values and swaps their contents.
    You must manipulate the values using dereferencing within the function while ensuring that the borrowing rules are followed (only one mutable reference to a given piece of data at a time).
    This challenge helps you practice safe, in-place modifications using references.
*/

fn swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(&mut (*a), &mut (*b));
}

fn main() {
    let mut a = 10;
    let mut b = 20;
    swap(&mut a, &mut b);
    println!("now a is {a} and b is {b}");
}
