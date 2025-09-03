/*
Create a function that takes a String as input, appends some text to it, and then returns the modified value.
*/

fn modified_string(s: &mut String) {
    s.push_str("ok");
}

fn main() {
    let mut s = String::from("hello ");
    modified_string(&mut s);
    println!("string is {s}");
}
