use std::rc::Rc;
struct Car{
    brand: Rc<String>,
}

impl Car{
    fn new(brand:Rc<String>) -> Car { Car{brand} }
    fn drive(&self) {
        println!("{} is driving", &self.brand);
    }

}

fn main() {
    let brand = Rc::new(String::from("BMW"));
    println!("pointers_count: {}", Rc::strong_count(&brand));

    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("pointers_count: {}", Rc::strong_count(&brand));
    }

    println!("My Car: {}", brand);
    println!("pointers_count: {}", Rc::strong_count(&brand));

    let brand2 = Rc::new(String::from("MacLaren"));
    let car2 = Car::new(brand2.clone());
    car2.drive();
    println!("pointers_count: {}", Rc::strong_count(&brand2));
}
