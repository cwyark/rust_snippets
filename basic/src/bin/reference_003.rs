/*
- Append a Suffix to a String
    Create a function named append_suffix that takes a mutable reference to a String and appends a specified suffix to it (for instance, “_suffix”).
    This task requires you to modify the underlying data without transferring ownership, demonstrating the proper use of mutable references.
*/

fn append_suffix(message: &mut String, suffix: &str) {
    message.insert_str(0, suffix);
}

fn main() {
    let mut s = String::from("world");
    let suffix = "hello ";
    append_suffix(&mut s, suffix);
    println!("message is {s}");
}
