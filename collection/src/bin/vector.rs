fn main() {
    let v = vec![0, 3, 8, 4, 7];
    // iter through v
    for i in &v {
        println!("i is {i}");
    }

    // iter through the vector as well, but use .iter()
    for i in v.iter() {
        println!("i is {i}");
    }
}
