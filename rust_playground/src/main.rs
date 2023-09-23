pub mod memory_management;
pub mod generics_structs_traits;
mod data_types;


fn main() {
    // hello();
    // memory_management::demo_mem_mangement();
    // memory_management::bad_mem_management();
    // list_contents(["a", "b", "c", "d", "e", "f"]);
    // memory_management::demo_mem_mangement();
    // generics_structs_traits::list_contents_any_type([1, 2, 3,]);

    let _car1 = generics_structs_traits::Car {
        make: String::from("Toyota"),
        model: String::from("Camry"),
        year: 2019,
    };
    let _car2 = generics_structs_traits::Car {
        make: String::from("Honda"),
        model: String::from("Accord"),
        year: 2018,
    };

    let _car3 = generics_structs_traits::Car {
        make: String::from("Nissan"),
        model: String::from("Altima"),
        year: 2017,
    };

    let mut _tuple_stuff = (1, "3", 4.0, 9, true, generics_structs_traits::Vehicle::V8);

    fn some () -> i32 {
        println!("random addition {}", 3+6);

        3+6
    }

    let some_fn = some();

    let some_vec:Vec<&str> = vec!["some"];

    println!("print fn {:#?}", some_fn);
    println!("len {:?}", some_vec.len());

    generics_structs_traits::list_contents_any_type([1, 2, 4]);

    let tuple = ("string", 2, false, generics_structs_traits::Vehicle::V4);

    println!("printing a string from a tuple {}", tuple.2);

    data_types::data_types::data_types_function();
}




fn _list_contents (list_array: [&str; 6]) {
    for item in list_array {
        println!("{}", item);
    }
}

fn _hello() {
    println!("Hello, world!");

    let mut vector_array = Vec::new();

    vector_array.push(1);
    // vector_array.push("3");
}





