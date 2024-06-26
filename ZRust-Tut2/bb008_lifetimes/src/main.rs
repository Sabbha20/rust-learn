
#[derive(Debug)]
struct Person{
    name : String,
}

impl Person{
    fn create(name:String) -> Self{
        Person{name}
    }


    fn get_name(&self) -> &str{
    // fn get_name<'l> (&'l self) -> &'l str{
        &self.name
    }
}

#[derive(Debug)]
struct Dog<'l>{ // here 'l is representing as long as the Dog object is there Person object will not be invalid
    name: String,
    owner: &'l Person,
}
impl<'a> Dog<'a>{
    fn create(name:String, owner:&'a Person) -> Self{
        Dog{name, owner}
    }
}

fn main() {
    println!("{}", get_str());

    let p1 = Person{name:String::from("Sabbha")};
    let d = Dog{name:String::from("Pup"), owner:&p1};

    println!("{:?}", d);

    let p2 = Person::create("Emma".to_string());
    let d2 = Dog::create("Tommy".to_string(), &p2);
    println!("{:?}", d2);

    let mut a:&str;
    {
        let p3 = Person::create("Mark".to_string());
        // a = p3.get_name();
        a = p2.get_name();
    }

    println!("{:?}", a);
    println!("{:?}", p2);

}

fn get_str() -> &'static str{ // &'static provides lifetime, as long as main fn lives
    "Sabbha is developer."
}