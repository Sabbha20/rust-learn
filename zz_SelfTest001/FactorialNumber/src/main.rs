use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut  input).expect("Cannot read the input");
    let num:u32 = input.trim().parse().expect("Please enter valid number");

    println!("Factorial of {num}: {:?}", factorial(num));
}


fn factorial(n:u32) -> u32{
    if n == 0{
        return 1
    }
    else {
        return n * factorial(n-1)
    }
}