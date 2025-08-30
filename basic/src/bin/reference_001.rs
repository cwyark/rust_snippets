/*
- Calculate Average from a Slice
    Write a function named calculate_average that accepts an immutable reference (a slice) to a list of i32 values (for example, &[i32]) and returns the average as an f64.
    Your function should iterate over the slice, sum the values, and compute the average.
    This exercise reinforces how to pass data by reference without taking ownership, allowing you to safely inspect the data
*/

fn calculate_average(numbers: &[i32]) -> f64 {
    let mut sum: u64 = 0;
    for v in numbers {
        sum += *v as u64;
    }
    sum as f64 / numbers.len() as f64
}

fn main() {
    let v = [0, 4, 6, 5, 7];
    let avg = calculate_average(&v);
    println!("v is {avg}");
}
