use traits::{Result, Add, Sub, operations_present};

fn main() {
    let add = Add {
        num1: 12,
        num2: 5,
        op: "+".to_string(),
        name: "Addition".to_string(),
    };

    let sub = Sub {
        num1: 14,
        num2: 10,
        op: "-".to_string(),
        name: "Subtraction".to_string(),
    };

    let operation_name = operations_present(&sub);

    format!("{:?}", operation_name);

    println!("Addition: {}", add.res());
    println!("Subtraction: {}", sub.res());
}
