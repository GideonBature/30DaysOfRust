fn main() {
    let mut name = String::from("Gideon");

    name.push_str(" Bature");

    println!("name: {}", name);

    let new_name = "Grace";
    let mut name_string = new_name.to_string();
    name_string.push_str(" Andrew Ajogi");

    println!("name: {}", name_string);
}
