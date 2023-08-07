pub mod data_types {
pub fn data_types_function () { // pub keyword here
	let int = 32_u32; // unsigned 
	let int_signed = -32_i32; // signed
	let int_out_of_nowhere = 32; // default int type
	let bool = true;
	let mut size = 3_isize; // isize

	let sim_array = [5, 6, 7, 8, 9, 10]; // array
	let sim_array_fixed: [i32; 3] = [3, 4, 5]; // array

	let new_size = sim_array.len(); // usize

	let two_decimal = 2.4_f32;


	let hello = " hello";
	const hello_static: &str = "hello static";
	let mut parent_string = String::from("parent");
	parent_string.push_str(hello);
	let child_string = hello;



	println!("parent string {:?}", hello);
	let vector = Vec::<String>::new();
	let vector_2: Vec<String> = Vec::new();
	let vector_3: Vec<i32> = vec![2, 3, 4, 5, -1, -3, -4];



}

}
