mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    pub mod sitting {
        pub fn arrange_sit() {}
    }
}

use crate::front_of_house::hosting;
pub use crate::front_of_house::sitting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    sitting::arrange_sit();
}
