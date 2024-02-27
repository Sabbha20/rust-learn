use crate::Citizen::Foreign;

#[allow(non_snake_case)]
#[allow(dead_code)]


fn main() {
    let person = Person{
        name: String::from("Sabbha"),
        age:32,
        occupation: String::from("Developer"),
        citizenship: Citizen::Indian
    };

    // println!("{:?}", person);
    println!("{}", person.describe());

    let coord = Point{ x:32, y:34};
    println!("{:?}", coord);

    let p2 = Person::create("Jennifer".to_string(), 32, "Actress".to_string(), Citizen::Foreign);
    println!("{}", p2.describe());
}

// #[derive(Debug)]
struct Person{
    name: String,
    age: u32,
    occupation: String,
    citizenship: Citizen,
}

impl Person {
    fn describe(&self) -> String{
        format!("{} is a {} years old {}, {:?}.", self.name, self.age, self.occupation, self.citizenship)
    }

    fn create(name:String, age:u32, occup:String, citizen:Citizen ) -> Self{
        Self{
            name,
            age,
            occupation:occup,
            citizenship:citizen
        }
    }

}

#[derive(Debug)]
enum Citizen{
    Indian,
    Foreign,
    Others
}

// Generics
#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}