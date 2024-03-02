fn main() {
//     Closures are like lambda functions
    let sum = |a:i32, b:i32| -> i32 {a+b};
    println!("{}", sum(2,3));

    let subtract = |x, y| {
        let c = x-y;
        c
    };
    println!("{}", subtract(3, 5));

    let multiply = |x, y| x*y;
    println!("{}", multiply(3, 5));

//     Closure as generics
    let gen = |x| println!("Msg received:\t{}", x);
    gen("Sabbha");
}
