// -------------------------------------------
// 	Generics
// -------------------------------------------
//



struct Point<T, U>{
    x: T,
    y: U
}
impl<T, U> Point<T, U> {
    fn create(x:T, y:U) -> Self{
        Point{x, y}
    }
}
impl<T, U> Point<T, U> {
    fn new(x:T, y: U) -> Point<T, U>{
        Point { x, y }
    }


}

impl Point<i32, i32>{
    fn show_msg(&self) {
        println!("Co-ordinate: ({}, {})", self.x, self.y)
    }
}



fn main() {

    let origin = Point {x : 0, y : 0};
    let point_a = Point{x:5, y: 9};
    let point_b = Point::new(4, 5);
    // println!("Origin: {:?}", origin);
    // println!("Point B: {:?}", point_b)
    point_b.show_msg();
    let point_c = Point::create(5, 7);
    point_c.show_msg();


}
