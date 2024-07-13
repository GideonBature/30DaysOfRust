pub trait MakeNoise {
    fn make_noise(&self) {
        println!("(silence)");
    }
}

pub struct Dog {}

pub struct Cat {}

impl MakeNoise for Dog {
    fn make_noise(&self) {
        println!("bark");
    }
}

impl MakeNoise for Cat {}
