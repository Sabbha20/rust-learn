
#[allow(unused_variables)]
// -------------------------------------------------
// 			- Functions
// 			- Code Blocks
// -------------------------------------------------
fn main() {
    println!("Functions\n============================================================");
    print_name("Sabbha is great!");
    let multi = multiplication(3, 2);
    println!("Multiplication: {multi}");
    let (add, sub, mul, div) = basic_math(5, 2);
    println!("Addition: {add}\tSubtract: {sub}\tProduct: {mul}\tDivision: {div}");
    let name = "Sabbha";
    // Code Block
    let full_name = {
        let f_name = "Sabbha";
        let l_name = "Mondal";
        format!("{f_name} {l_name}")
    };

    print_name(name);
    println!("============================================================");
}

fn print_name(s: &str) {
    println!("Print Msg: {s}");
}

fn multiplication(n1: i32, n2: i32) -> i32 {
    n1 * n2
}

fn basic_math(n1: i32, n2: i32) -> (i32, i32, i32, i32) {
    (n1 + n2, n1 - n2, n1 * n2, n1 / n2)
}