fn main() {
    // To coerce a reference (&T) or mutable reference (&mut T)
    let my_num = 10;
    let my_num_ptr: *const i32 = &my_num;

    let mut my_mutable_num = 15;
    let my_mutable_num_ptr: *mut i32 = &mut my_mutable_num;

    let my_string = String::from("hello");
    let my_second_string = my_string;

    unsafe {
          println!("{:?}", my_mutable_num_ptr.as_ref().expect("error"));  
    }

    // consume a box pointers
    let box_var = Box::new(5);
    let box_ptr: *mut i32 = Box::into_raw(box_var);

    unsafe {
        println!("{:?}", box_ptr.as_ref().expect("an error occured"));
        drop(Box::from_raw(box_ptr));
        println!("{:?}", box_ptr.as_ref().expect("an error occured"));
    }

    // you can create scopes using brackets
   
     let mut parent_string = String::from("Parent");

    &mut parent_string.push_str("child");

    let child_string = &parent_string;

    println!("{:?}", parent_string);

   


    // lifetimes
    lifetime_fn(&parent_string);



}


fn lifetime_fn<'a> (item: &'a str) {
    println!("printing item {}", item)
}
