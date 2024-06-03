use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut  input).expect("Cannot read the input");
    let num:i32 = input.trim().parse().expect("Please enter valid number");

    println!("Factorial of {num}: {:?}", factorial(num));
}


fn factorial(n:i32) -> i32{
    if n == 0{
        return 1
    }
    else {
        return n * factorial(n-1)
    }
}