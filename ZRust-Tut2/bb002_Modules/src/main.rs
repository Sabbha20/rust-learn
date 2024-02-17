#[allow(non_snake_case)]
use crate::patterns::my_pat::print_pat;
use crate::play_mod::{watch_movie, listen_audio};
use rand::Rng;
mod play_mod;
mod patterns;
fn main() {
    play_mod::watch_movie("Matrix");
    play_mod::listen_audio("Matrix_theme");

    let num1 = 50;
    let num2 = 35;
    println!("Sum of {} and {} is {:?}", num1, num2, math_cal::mSum(num1, num2));
    println!("Difference of {} and {} is {:?}", num1, num2, math_cal::mSub(num1, num2));
    println!("Multiplication of {} and {} is {:?}", num1, num2, math_cal::mMul(num1, num2));
    println!("Division of {} and {} is {:?}", num1, num2, math_cal::mDiv(num1, num2));
    println!("XOR of {} on {} is {:?}", num1, num2, math_cal::extra_cal::mXOR(num1, num2));
    println!("OR of {} on {} is {:?}", num1, num2, math_cal::extra_cal::mOR(num1, num2));
    println!("AND of {} on {} is {:?}", num1, num2, math_cal::extra_cal::mAND(num1, num2));
    println!("Mod of {} and {} is {:?}", num1, num2, math_cal::extra_cal::m_mpd(num1, num2));

    print_pat("Sabbha");
    watch_movie("Iron Man");
    listen_audio("Salaar");

    println!("\n======================================================================\n");
    let mut rng = rand::thread_rng();
    let a:i32 = rng.gen();
    println!("RandomNumber:\t{:?}", a);

}

mod math_cal{
    pub fn mSum(n1:i32, n2:i32) -> i32{
        n1+n2
    }
    pub fn mSub(n1:i32, n2:i32) -> i32{
        n1-n2
    }
    pub fn mMul(n1:i32, n2:i32) -> i32{
        n1*n2
    }
    pub fn mDiv(n1:i32, n2:i32) -> i32{
        n1/n2
    }

    pub mod extra_cal{
        pub fn mXOR(n1:i32, n2:i32) -> i32{
            n1^n2
        }
        pub fn mOR(n1:i32, n2:i32) -> i32{
            n1|n2
        }
        pub fn mAND(n1:i32, n2:i32) -> i32{
            n1&n2
        }
        pub fn m_mpd(n1:i32, n2:i32) -> i32{
            n1%n2
        }
    }
}
