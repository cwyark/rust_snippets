fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("You can't divide by zero!".to_string())
    } else {
        Ok(a / b)
    }
}

fn try_divide() -> Result<i32, String> {
    let ret = divide(10, 3)?;
    Ok(ret)
}

fn main() {
    match divide(10, 0) {
        Ok(result) => println!("Result is {result}"),
        Err(e) => println!("Error: {e}"),
    }
    let a = divide(10, 2).unwrap();
    println!("a is {a}");
    let b = divide(10, 0).expect("division failed");
    println!("b is {b}");
    let c = try_divide().unwrap();
    println!("c is {c}");
}
