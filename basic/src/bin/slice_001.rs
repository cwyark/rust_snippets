/*
• Challenge 1: First Word Extractor
    Write a function that takes a string slice (of type &str) and returns a slice corresponding to the first word in the string.
    For example, given the input "hello world", the function should return "hello".
    Hint: Iterate through the characters of the slice until you find a space, and then return the appropriate substring slice.
*/

fn first_word(s: &str) -> &str {
    let mut space_idx = 0;
    for (idx, c) in s.char_indices() {
        if c == ' ' {
            space_idx = idx;
            break;
        }
    }
    &s[0..space_idx]
}

fn main() {
    let s = "hello world";
    let w = first_word(s);
    println!("first word is {w}");
}
