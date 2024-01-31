// -------------------------------------------------
// 			- Compound Data Types
//	 			- &str and String
// 				- Arrays
// 				- Vectors
// 				- Tuples
// 				- Empty Tuple
// -------------------------------------------------
//
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    //  &str and String
    let fixed_str = "Sabbha";
    let mut flexible_str = String::from("Sabbha is great. ");

    flexible_str.push('O');
    println!("flexible_str:\t{flexible_str}");

    // Arrays
    let mut ar1 = [1, 2, 3, 4, 5];
    println!("ar1:\t{:?}", ar1);
    let num = ar1[3];
    println!("num:\t{num}");
    let ar2 = [0; 10];
    println!("ar2:\t{:?}", ar2);

    // Vectors
    let v1 = vec![1, 2, 3, 4];
    let vn = v1[2];
    println!("v1:\t{:?}", v1);
    println!("vn:\t{:?}", vn);

    // Tuples
    let my_info = ("Name", "Sabbha", "Salary", 75000, "Age", 31);
    let my_sal = my_info.3;
    println!("my_info:\t{:?}", my_info);
    println!("my_sal:\t{:?}", my_sal);


    let unit = ();
}