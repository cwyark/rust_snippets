/*
- Write a function that takes ownership of a String and prints it.
*/

fn take_ownership(s: String) {
    // s'value is moved to text
    let text = s;
    println!(
        "s has been taken and assigned to text. s is not available now! s = {}",
        text
    );
} // s will not out of scope and not available.

fn main() {
    // String type is allocated under heap.
    let s = String::from("hello world !");

    take_ownership(s);
}
