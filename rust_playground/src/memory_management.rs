pub fn demo_mem_mangement() {
    let mut s = String::from("Demo memory management");
    s.push_str(", world!");
    println!("{}", s);
}

pub fn bad_mem_management() {
    let s1 = String::from("Bad memory management");
    let s2 = s1;
    println!("{}, fixed!", s2);
}