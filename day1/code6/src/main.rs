fn main() {
    let first_name = String::from("Gideon");
    let last_name = String::from("Bature");

    full_name(&first_name, &last_name);

    println!("my name is {} {}", first_name, last_name);
}

fn full_name(f_n: &String, l_n: &String) {
    println!("my name is {} {}", f_n, l_n);
}
