use rand::{Rng, thread_rng};
use rand::distributions::{Alphanumeric, Bernoulli};
fn main() {
    let mut rng = thread_rng();
    let i: i32 = rng.gen();

    println!("i: {:?}", i);
    println!("Bounded int: {:?}", rng.gen_range(0..=100));
    println!("Bounded float: {:?}", rng.gen_range(0.0..100.0));

    let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).map(char::from).collect();
    let rand_str: &str = &rand_string;
    println!("Generated String: {:?}", rand_string);
    println!("Generated &str: {:?}", rand_str);
}
