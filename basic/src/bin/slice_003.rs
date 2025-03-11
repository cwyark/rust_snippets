/*
• Challenge 3: In‐Place Slice Reversal
    Write a function that takes a mutable slice of integers (e.g., &mut [i32]) and reverses the order of the elements in place without allocating additional memory
    (i.e., do not use Vec to solve this).
    For example, given the slice [1, 2, 3, 4, 5], after reversal it should be [5, 4, 3, 2, 1].
    This problem will allow you to practice working with mutable slices and in‑place data manipulation.
*/

fn in_place_reverse(v: &mut Vec<i32>) -> &Vec<i32> {
    let mut start = 0;
    let mut end = v.len() - 1;
    loop {
        if start == end {
            break;
        }
        v.swap(start, end);
        start += 1;
        end -= 1;
    }
    v
}

fn main() {
    let mut v = vec![0, 6, 4, 5, 9];
    let reversed = in_place_reverse(&mut v);
    println!("v is {:?}", reversed);
}
