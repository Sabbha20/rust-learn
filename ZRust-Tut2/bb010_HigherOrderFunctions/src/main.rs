fn main() {
    println!("Hello, world!");

    let square = |x:i32| x*x;
    println!("Square: {}", square(5));

    apply(square, 5);

    // Problem Def:
    /*
    Calculate the sum of all squares below 500
    only for even numbers
    */
    let limit = 500;
    let mut sum = 0;
    for i in 0..{
        let sq = i*i;
        if sq > limit {break ;}
        else {
            if is_even(sq){
                sum = sum + sq;
            }
        }
    }
    println!("Sum of all squares below 500 without HOF : {sum}");


    // Using HOF

    let sum2 =
        (0..).map(|x| x*x)                   // All natural numbers squared
            .take_while(|&x|x<=limit)        // Below upper limit
            .filter(|x| is_even(*x))        // That are even
            .fold(0, |my_add, x| my_add+x);          // Sum them
    println!("Sum of all squares below 500 with HOF : {sum2}");

}


fn is_even(i:i32) -> bool {
    i%2==0
}

fn apply(f: fn(i32) -> i32, a: i32){
    println!("Result: {}", f(a));
}