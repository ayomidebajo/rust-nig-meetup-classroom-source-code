use std::fmt::Debug;


#[allow(unused)]
fn main() {
    let list = vec![1, 2, 3, 4, 5];
    let list_str = vec!["a", "b", "c", "d"];
    let my_pc = Pc {
        name: String::from("Hamed"),
        model: String::from("Hp")
    };
    let my_pc1 = Pc {
        name: String::from("Hamed"),
        model: String::from("Hp")
    };
    let my_pc2 = Pc {
        name: String::from("Hamed"),
        model: String::from("Hp")
    };
    let struct_list = vec![my_pc, my_pc1, my_pc2];

    // our custom lamp with the enum
    let my_custom_lamp: Customlamp<String> = Customlamp { name: String::from("lontor"), lamp: Lamp::BedsideLamp(String::from("lontor lamp")) };

    // printing the value wrapped 
   println!("some enum {:?}", my_custom_lamp.lamp.print_out_lamp::<String>());

    // list_numbers_only(&list);
    generic_list(&struct_list);
    
}


#[allow(unused)]
#[derive(Debug)]
struct Pc {
    name: String,
    model: String
}


#[allow(unused)]
#[derive(Debug)]
struct Customlamp<T> {
    name: String,
    lamp: Lamp<T>
}

fn _list_numbers_only (list: &[i32]) {
    for item in list {
        println!("item {item}");
    }
}

fn generic_list<T: Debug> (list: &[T]) {
    for item in list {
        println!("item {item:?}");
    }
}


#[allow(unused)]
#[derive(Debug)]
enum Lamp<T> {
    TableLamp,
    BedsideLamp(T),
    HandLamp
}

impl Lamp<String> {
    fn print_out_lamp<T> (&self) -> String {
        match self {
            Lamp::TableLamp => String::from("Table lamp"),
            Lamp::BedsideLamp(u) => String::from(u),
            Lamp::HandLamp => String::from("hand lamp")
        }
    }
}