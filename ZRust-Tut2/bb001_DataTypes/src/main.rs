
// Here we will do STRINGS
fn main() {
    /*
    &str -  it's a slice of string in memory reference, immutable
    String - it's a string object, immutable
    */
    let my_name = "Sabbha";

    // We use &'static for maintaining the life span of String variable -> my_static_name
    let my_static_name: &'static str = "Sabbha";

    println!("{}", my_name);
    println!("{}",my_static_name);

    let mut my_dog = String::from("Fluffy");

    // let my_sentence = format!("Hi, I am {} and this is {}", my_name, my_dog);
    // println!("{}", my_sentence);
    println!("1 my_dog: \t{}", my_dog);

    my_dog.push(' ');// We can add character
    my_dog.push_str("the dog."); // We can add string

    let my_sentence = format!("Hi, I am {} and this is {}", my_name, my_dog);
    println!("{}", my_sentence);

    println!("2 my_dog: \t{}", my_dog);
    println!("my_name.(): \t{:?}", my_name.chars().nth(2));

for i in 1..6{
    println!("{}",i);
    say_hi();
}



}

fn say_hi(){
    print!("Hi there!")
}