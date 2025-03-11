/*
• Challenge 2: Maximum Value in a Slice
    Create a function that accepts an immutable slice of integers (e.g., &[i32]) and returns a reference to the maximum value in that slice.
    Your function should handle cases where the slice is empty by either returning an Option type or by using a precondition.
    This problem will help you practice iterating over slices and returning a reference tied to the slice’s lifetime.

*/

fn main() {
    let v = vec![0, 3, 7, 5, 8];
    let mut max = 0;
    for i in &v {
        if *i > max {
            max = *i;
        }
    }
    println!("max number of the array is {}", max);
}
