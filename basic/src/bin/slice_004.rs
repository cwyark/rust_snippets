fn first_word(s: &String) -> u8 {
    let bytes = s.as_bytes();
    bytes[0]
}

fn main() {
    let s = String::from("hello world");
    let _w = first_word(&s);
    println!("first word is {}", _w);

    // String slicing
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello is {}", hello);
    println!("world is {}", world);
}
