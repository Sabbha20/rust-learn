
fn main() {

//     -----------------------------------------------------------
//          - Loops
//          - For loops
//          - While loops
//     -----------------------------------------------------------

    // loops
    loop {
        println!("Simple loop");
        break; // Creates an infinite loop that runs repeatedly until explicitly terminated using break.

    }

    'outer: loop {
        println!("Simple outer loop");
        break 'outer;
    }

    // We can assign loops to a variable
    let a = loop {
        break 5;
    };
    println!("a:\t{:?}", a);

    let v = vec![23, 45, 65, 76, 29];
    for i in v{
        println!("{i}");
    }

    let mut num = 0;
    while num < 11 {
        println!("num:\t{num}");
        num = num + 1;
    }

}


// // fn main() {
// //    let three = 3;

// //    if three {
// //        println!("Number was three");
// //    }
// // }

// // fn main() {     let x: u64 = 4_294_967_296;     let y = x as u32;     if x == y as u64 {         println!("x equals y.");     } else {         println!("x does not equal y.");     } }