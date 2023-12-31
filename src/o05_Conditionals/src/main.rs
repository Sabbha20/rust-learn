
fn main() {
    println!("Conditionals\n============================================================");

    let num = 40;
    if num <= 50{
        println!("{num} <= 50")
    } else {
        println!("{num} > 50")
    }

    let marks = 95;
    // let mut grade = 'N';
    // ______________________________________ //
    // if marks >= 90 {
    //     grade = 'A';
    // } else if marks >= 80{
    //     grade = 'B';
    // } else if marks >= 70{
    //     grade = 'C';
    // } else {
    //     grade = 'F';
    // }
    // __The above code can be written as____ //

    let grade = if marks >= 90 {
        'A'                         // All the return type values must be same : like here its char
    } else if marks >= 80{
        'B'
    } else if marks >= 70{
        'C'
    } else {
        'F'
    };
    println!("Marks: {marks}\tGrades:{grade}");
    println!("============================================================");
    println!("############################################################");
    println!("Match\n============================================================");
    let marks = 75;
    // let mut grade = 'N';

    // ______________________________________ //
    // match marks {
    //     90..=100 => grade = 'A',// 1st arm    // 90..100 --> range [90-99] and 90..=100 --> range [90-100]
    //     80..=89 => grade = 'B',// 2nd arm    // These are known as arms
    //     70..=79 => grade = 'C',
    //     _ => grade = 'F'  // last arm        // default arm --> matches all the other conditions not mentioned
    //     // There should not be any condition after default arm, they would be unreachable conditions
    // };
    // __The above code can be written as____ //
    let gradem = match marks {
        90..=100 => 'A',// 1st arm    // 90..100 --> range [90-99] and 90..=100 --> range [90-100]
        80..=89 => 'B',// 2nd arm    // These are known as arms
        70..=79 => 'C',
        _ => 'F'  // last arm        // default arm --> matches all the other conditions not mentioned
        // There should not be any condition after default arm, they would be unreachable conditions
    };
    println!("Marks: {marks}\tGrades:{gradem}");
    println!("============================================================");
}