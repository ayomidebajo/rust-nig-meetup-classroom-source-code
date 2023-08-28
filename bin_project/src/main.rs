
use bin_project::from_lib;

pub mod module1;

// pub mod module1::module1_child;


fn main() {
    println!("Hello, world!");
    module1::first_fn_in_mod1();

    from_lib()
}


