
trait Summable<T>{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>{
    fn sum(&self) -> i32 {
        let mut total = 0;
        for i in self{
            total = total + *i;
        }
        total
    }
}
fn main() {
    let v = vec![1,2,3,4,-5];
    print!("sum of {:?} is {:?}", v, v.sum());
}
