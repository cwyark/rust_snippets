use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, i32> = Default::default();
    map.insert("1", 1);
    println!("map is {:?}", map);
}
