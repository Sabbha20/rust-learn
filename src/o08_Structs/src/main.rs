// -------------------------------------------
// 	Structs and its Types
// -------------------------------------------

struct Person{
    name: String,
    age: u32,
    nationality: String,
    occupation: String,
}

fn main() {
    // Instance of a person
    let person1 = Person{               // All fields must be assigned
        name: String::from("Sabbha"),
        age: 30,
        nationality: "Indian".to_string(),
        occupation: "Developer".to_string(),
    };
    let p1_name = person1.name; // p1_name takes ownership, as teh value is heap allocated -> Partial move
    // println!("Name:{:?}", person1.name)
    // person1.age = 31; // The fields are by default immutable
    println!("Name:{:?}", p1_name);

    let mut p2 = Person{
        name:"Ajay".to_string(),
        age: 50,
        nationality: "Indian".to_string(),
        occupation: "Actor".to_string(),
    };
    println!("P2 Age:{:?}", p2.age);
    p2.age = 54; // Here the person p2 is mutable
    println!("After P2 Age:{:?}", p2.age);
    let p2_name = p2.name.clone(); // Ownership still remains with p2.name
    let p2_occ = &p2.occupation;
    println!("P2 Name:{:?}", p2.name);
    println!("p2_name:{:?}", p2_name);
    println!("P2 occupation:{:?}", p2.occupation);
    println!("p2_occ:{:?}\n\n\n\n", p2_occ);

    let mut p3 = Person{
        name: "SRK".to_string(),
        ..p2 // borrow from p2
    };
    let p3_name = p3.name.clone(); // Ownership still remains with p2.name
    let p3_occ = &p3.occupation;
    let p2_age = &p2.age;
    let p3_age = &p3.age;
    println!("P3 Name:{:?}", p3.name);
    println!("p3_name:{:?}", p3_name);
    println!("P3 occupation:{:?}", p3.occupation);
    println!("p3_occ:{:?}", p3_occ);
    println!("P3 age:{:?}", p3.age);
    println!("p3_age:{:?}\n\n\n", p3_age);
    println!("P2 Name:{:?}", p2.name);
    println!("p2_name:{:?}", p2_name);
    //----------------------------------------------------------------
    // Since p3 borrowed string value from p2, a heap allocated value,
    // we cannot use in p2 anymore.
    // But we can use the primitive datatype
    // ---------------------------------------------------------------
    // println!("P2 occupation:{:?}", p2.occupation);
    // println!("p2_occ:{:?}", p2_occ);
    println!("P2 age:{:?}", p2.age);
    println!("p2_age:{:?}", p2_age);

    // -------------------------------------------
    // 	Tuple Structs and its Types
    // -------------------------------------------
    // let pt_2D = (2, 3);
    // let pt_3D = (4, 6, 7);
    // #[derive(Debug)]
    struct Point_2D(i32, i32);
    // #[derive(Debug)]
    struct Point_3D(i32, i32, i32);

    
    
    let pt1 = Point_2D(2, 3);
    let p2 = Point_3D(3, 4, 5);
    
    // println!("Point_2D: {:?}", pt1);
    // println!("Point_3D: {:?}", p2);

    // Unit Structs
    struct ABC;
}