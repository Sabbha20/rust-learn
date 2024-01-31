// -------------------------------------------
// 	Structs with methods
// -------------------------------------------

struct Animal{
    name: String,
    animal_type: String,
    kind: String, // herbivores, carnivores, omnivores
    limbs: i32,
    age: f32,
}

impl Animal{
    //------------------------------------------------------------------
    // Associated functions: It has no 'self' keyword
    //------------------------------------------------------------------
    // These functions are not called using dot syntax, like others.
    // They are called differently
    fn GST() -> u32 {
        120 // In ₹
    }

    fn sold(&self, selling_price:f32) -> f32{
        // Associated functions are called with ::
        (self.age*selling_price) + Animal::GST() as f32
    }

    //---------------------------------
    // Constructor functions:
    //---------------------------------
    // Constructor functions helps creating
    // new instances with ease.
    fn birth(animal_name: String, animal_type: String, cho:String) -> Self {
        Self{
            name:animal_name,
            animal_type:animal_type,
            kind:cho,
            limbs:4,
            age:0.1,
        }
    }

    //---------------------------------
    // Methods functions:
    //---------------------------------
    // Where the methods or functions are defined for a structs(class)
    fn define(&self){
        println!("The {} name is {} and its {} with {} limbs and is {} years old.",
                 self.animal_type, self.name, self.kind, self.limbs, self.age
        );
    }

    fn add_age(&mut self, years:f32){
        self.age+=years
    }

    fn magically_change_to_animal(mut self, type_of_animal:String) -> Self{
        self.animal_type = type_of_animal;
        self
    }

}
fn main() {
    let mut cat = Animal{
        name: "Kit".to_string(),
        animal_type: "cat".to_string(),
        kind: "carnivores".to_string(),
        limbs: 4,
        age: 2.0,
    };

    cat.define();
    // To make changes in cat's age, we have to make the cat mutable
    cat.add_age(2.3);
    cat.define();
    // Owner of the animal is no longer cat, now its a dog
    let dog = cat.magically_change_to_animal("dog".to_string());
    dog.define();
    // cat.define(); // ERROR
    // Selling the dog
    let dog_sold = dog.sold(59.9);
    println!("The {} {} is sold for {}\n\n", dog.animal_type, dog.name, dog_sold);

    // Creating instances using constructor functions
    // Birth of an animal, 👻::
    let mut rabbit = Animal::birth("Bunny".to_string(), "rabbit".to_string(), "herbivore".to_string());
    rabbit.define();
    rabbit.add_age(2.5);
    rabbit.define();
}

// Notes:
// -------------------------------
// When declaring self in functions - its considering the object(instance of struct) as a whole, it is borrowed
// When declaring &self in functions - its considering the reference of the object, it is not borrowed