fn main() {
    let name = String::from("Gideon Bature");

    let f_name: &str = &name[0..6];
    let l_name: &str = &name[7..13];

    let s: &String = &name;

    println!("my name is {}", *s);

    println!("my first name is {}, and my last name is {}", f_name, l_name);
}
