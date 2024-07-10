fn main() {
    let first_name = String::from("Gideon");
    let middle_name = String::from("Funom");
    let last_name = String::from("Bature");

    let name = first_name + " " + &middle_name + " " + &last_name;

    println!("name: {}", name);
}
