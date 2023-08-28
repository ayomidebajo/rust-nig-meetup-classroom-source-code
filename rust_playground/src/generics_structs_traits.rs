use std::fmt::Debug;

pub fn list_contents_any_type<T: Debug>(list_array: [T; 3]) {
    for item in list_array {
        println!("{:#?}", item);
    }
}

#[allow(unused)]
pub fn list_contents_structs(list_array: [Car; 3]) {
    for item in list_array {
        println!(
            "This car name is {:?}, its model is {:?} and it was manufactured in the year {:?}",
            item.make, item.model, item.year
        );

        let vector = vec![2, 4, 5];
        let some_string = String::from("strings");
        let car_one = Car::new(String::from("Toyota"), String::from("Corolla"), 2009);

        let boxed = Box::new(4);
    }
}

#[allow(unused)]
fn list_numbers(list_num: &[i32]) {
    for item in list_num {
        println!("item: {item}");
    }
}

// A generic function
#[allow(unused)]
fn list_any_type<T: Debug>(list: &[T]) {
    for item in list {
        println!("item {item:?}");
    }
}


// accepts a lifetime
#[allow(unused)]
fn accept_lifetime<'a> (arg: &'a str) -> &'a str {
    arg
}


// lifetime syntax
// {
//     &str // a reference 
//     &'a str // a reference with an explicit lifetime
//     &'a mut str // a mutable reference with an explicit lifetime
// }

trait Gender {
    fn current_gender(&self) -> String;
    fn previous_gender(&self) -> String;
}

fn gender<T: Gender> (gender: impl Gender) {
    gender.current_gender();
}




#[allow(unused)]
struct FootballPlayer {
    gender: String,
    name: String,
    height: u32,
    age: u32,
    previous_gender: Option<String>
}

#[allow(unused)]
struct Point <T> {
    x: T,
    y: T
}


struct PartOfSentence <'a> {
    part: &'a str
}


impl Gender for FootballPlayer {
    fn current_gender(&self) -> String {
        self.gender.clone()
    }

    fn previous_gender(&self) -> String {
        self.previous_gender.as_ref().expect("expected a string").to_string()
    }
}

#[allow(unused)]
enum Lamp<T> {
    TableLamp,
    BedsideLamp(T),
    HandLamp,
}

#[derive(Debug)]
pub struct Car {
    pub make: String,
    pub model: String,
    pub year: u32,
}

impl Car {
    pub fn new(make: String, model: String, year: u32) -> Self {
        Self { make, model, year }
    }
}

#[derive(Debug)]
pub enum Vehicle {
    V8,
    V6,
    V4,
}

impl Vehicle {
    pub fn price(&self) -> u32 {
        match self {
            Vehicle::V4 => {
                32;
                return 45;
            }
            Vehicle::V6 => 38,
            // Vehicle::V8 => 44,
            _ => 0,
        }
    }
}
