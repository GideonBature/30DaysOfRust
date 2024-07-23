pub trait Result {
    fn operation(&self) -> String;

    fn res(&self) -> String {
        format!("Please Implement the res method on the {} operation", self.operation())
    }
}

pub struct Add {
    pub num1: i32,
    pub num2: i32,
    pub op: String,
    pub name: String,
}

impl Result for Add {
    fn operation(&self) -> String {
        format!("{} Operation", self.name)
    }

    fn res(&self) -> String {
        format!("{} {} {} = {}", self.num1, self.op, self.num2, self.num1 + self.num2)
    }
}

pub struct Sub {
    pub num1: i32,
    pub num2: i32,
    pub op: String,
    pub name: String,
}

impl Result for Sub {
    fn operation(&self) -> String {
        format!("{}", self.name)
    }
    /*
    fn res(&self) -> String {
        format!("{} {} {} = {}", self.num1, self.op, self.num2, self.num1 - self.num2)
    }
    */
}

pub fn operations_present(item: &impl Result) {
    println!("Operations: {}", item.operation());
}
