//     -----------------------------------------------------------
//          - Ownerships
//     -----------------------------------------------------------

#[allow(unused_variables)]

fn main() {

    let s1 = String::from("world");
    // let s2 = s1; // s2 takes ownership, so we clone
    let s2 = s1.clone();
    println!("s1:\t{s1}");

    // Ownership does not work with primitive type - int, float, boolean, char
    // here the values are copied and not moved.
    let n1 = 33;
    let n2 = n1;
    println!("n1:\t{n1}");
    println!("n2:\t{n2}");

    let vec_1 = vec![1, 2, 3];
    // takes_ownership(vec_1); // Same way the functions takes the ownership of the variable , so we clone
    takes_ownership(vec_1.clone());
    println!("vec_1:\t{:?}", vec_1);

    let vec_2 = gives_ownership();
    println!("vec_2:\t{:?}", vec_2);

    // let vec_3 = takes_and_gives_ownership(vec_2);
    // println!("vec_3:\t{:?}", vec_3);

    // consider changing this parameter type in function `takes_and_gives_ownership`
    // to borrow instead if owning the value isn't necessary
    let vec_3 = takes_and_gives_ownership(vec_2.clone());
    println!("vec_3:\t{:?}", vec_3);
    println!("2 vec_2:\t{:?}", vec_2);

    let x = 20;
    stack_only(x);
    println!("Value of x:\t{x}");

}

fn takes_ownership(vec: Vec<i32>){
    println!("vec:\t{:?}", vec);
}

fn gives_ownership() -> Vec<i32>{
    vec![4, 5, 6]
}

fn takes_and_gives_ownership(mut vec:Vec<i32>) -> Vec<i32>{
    vec.push(10);
    vec
}

fn stack_only(mut var:i32){
    println!("var in fn:\t{var}");
    var = 56;
    println!("var 2 fn:\t{var}");
}