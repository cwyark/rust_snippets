#[derive(Debug)]
struct User {
    name: String,
}

impl User {
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let user_1 = User {
        name: String::from("John"),
    };

    println!("user_1 is {user_1:?}");
    println!("user_1's name is {}", user_1.name);
    let mut user_2 = User { ..user_1 };

    user_2.name = "Marry".to_string();
    println!("user_2 is {user_2:?}");
    let user_2_name = user_2.get_name();
    println!("user_2's name is {user_2_name}");
}
