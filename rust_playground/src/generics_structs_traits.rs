use std::fmt::Debug;

pub fn list_contents_any_type<T: Debug>(list_array: [T; 3]) {
    for item in list_array {
        println!("{:#?}", item);
    }
}

pub fn list_contents_structs(list_array: [Car; 3]) {
    for item in list_array {
        println!(
            "This car name is {:?}, its model is {:?} and it was manufactured in the year {:?}",
            item.make, item.model, item.year
        );
    }
}

#[derive(Debug)]
pub struct Car {
    pub make: String,
    pub model: String,
    pub year: u32,
    // pub vector_array: Vec<u32>,
    // pub bool: bool,
    // pub char: char,
    // pub engine: Vehicle,
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
    pub fn price (&self) -> u32 {
        match self {
            Vehicle::V4 => {
                32;
                return 45;
            },
            Vehicle::V6 => 38,
            // Vehicle::V8 => 44,
            _ => 0

        }
    }
}
