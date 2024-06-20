fn main() {
    println!("Hello, world!");
    let s = String::from("Hello, world!");

    println!("{:?}", length_of_last_word(s));
}
pub fn length_of_last_word(s: String) -> i32 {
    // let s1 = s.trim();
    // let words:Vec<&str> = s1.split_whitespace().collect();
    // if let Some(last_word) = words.last(){
    //     last_word.len() as i32
    // } else {
    //     0
    // }
    s.split_whitespace().last().unwrap().len() as i32
}