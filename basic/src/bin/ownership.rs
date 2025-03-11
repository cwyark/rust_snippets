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

    // s is moved to my_text. so s si not invalid for now.
    let my_text = s;
    println!("text is {my_text}");

    // clone a my_text copy and assign to my_text_clone
    let my_text_clone = my_text.clone();

    println!("my_text is still available ! my_text is {my_text}");
    println!("my_text_clone is {my_text_clone}");

    // immutable variable like int, they are copied on assignment.
    let my_num = 5;
    println!("my_num is {my_num}");

    let my_num_clone = my_num;
    println!("my_num_clone is {my_num_clone}");

    println!("my_text_clone is going to passed to take_ownership function.");

    take_ownership(my_text_clone);
}
