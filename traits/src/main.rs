// Defining traits
trait AnimalBehaviour {
    // Trait with default implementation
    fn eat(&self) {
        println!("Eating food...")
    }

    // Trait with no default implementation
    fn greet(&self) {}

}

trait Behaviour {
    // associated method
    fn movement() {}
    // instance method
    fn breathe(&self) {}
}

impl Behaviour for Human {
    fn movement() {
        println!("I can move with my legs, bro")
    }

    fn breathe(&self) {
        println!("Breathe in, Breathe out");
    }
}


struct Human {
    name: String,
}

#[derive(Copy, Clone)]
struct Numb {
    interger: i32
}

// implementing a struct
impl Human {
    fn new(name: String) -> Human {
        Human { name }
    }
}

// Implemenating a trait on an type
impl AnimalBehaviour for Human {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn eat(&self) {
        println!("This is a human eating");
    }
}

struct Dog {
    name: String,
}

impl Dog {
    fn new(name: String) -> Dog {
        Dog { name }
    }
}

// Implemenating a trait on an type
impl AnimalBehaviour for Dog {
    fn greet(&self) {
        println!("Woof, my name is {}", self.name)
    }
}

// Using functions with traits
fn all_animals_eat(animals: Vec<&dyn AnimalBehaviour>) {
    for animal in animals.iter() {
        animal.eat()
    }
}

// Using functions with traits
fn _all_animals_sound(animal: impl AnimalBehaviour) {
    animal.greet();
    animal.eat();
}

// Using generics with trait bound
fn generic_with_trait_bound<T>(animals: Vec<T>)
where
    T: AnimalBehaviour,
{
    for animals in animals.iter() {
        animals.greet();
        // animals.eat();
    }
}

fn main() {
    // using instance methods
    let human = Human::new("Bola".to_string());
    human.breathe();

    // using associated methods
    Human::movement();

    // creating a collection of objects that use the trait
    let human1 = Human::new("Marho".to_string());
    let dog1 = Dog::new("Max".to_string());
    let human2 = Human::new("Bekka".to_string());
    let animals: Vec<&dyn AnimalBehaviour> = vec![&human1, &dog1, &human2];
    // let all_animals = vec![human1, dog1];
    // let rand_vec = vec![1, 2];

    // Using function
    all_animals_eat(animals);

    // Illustrating the difference between using traits and using generic + trait bound
    // The major advantage here is the we are sure that the generic object will have the specific behavoir we want.
    // In this case, animal behaviour

    let animals = vec![
        Human::new("Marho".to_string()),
        Human::new("Bekka".to_string()),
    ];

    // Using trait bound + generic functions
    generic_with_trait_bound(animals);

    // Illustration of the limitation of generics
    // This code is not suppose to compile, when dog is uncommented !
    let _animals = vec![
        Human::new("Marho".to_string()),
        Human::new("Bekka".to_string()),
        // Dog::new("Max".to_string()),
    ];
    // using into while specifying the type already on the "left hand side"
    let _stri: String = "st".into();

    // using turbo fish syntax.
    // In this case we don't need to specify the type on the left hand side.
    let _stri = Into::<String>::into("str");

    // Illustrait the safeguard that traits offer
    let _wrong_objects = vec![String::from("wrong"), String::from("code")];

    // This function should give an uncompiled error when uncommented
    // generic_with_trait_bound(wrong_objects)
}
