fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyx");

    let result = longest(string1.as_str(), string2.as_str());

    println!("the longest string is {}", result);
}

// &i32 // a reference
// &'a i32 // a reference with an explivit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
