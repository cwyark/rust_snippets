fn calculate_len(s: &String) -> usize {
    let text = s;
    text.len()
}

fn change_str(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello world");
    let length = calculate_len(&s);
    {
        let r = &s;
        println!("r is {}", r);
    }
    println!("s size is {}", length);
    println!("s is {}", s);
    change_str(&mut s);
    println!("modified s is {}", s);
}
