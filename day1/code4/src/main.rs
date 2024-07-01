fn main() {
    let name = String::from("Gideon");
    let full = add_prefix(name);
    println!("{full}");
}

fn add_prefix(mut name: String) -> String {
    name.push_str(" Bature");
    name
}
