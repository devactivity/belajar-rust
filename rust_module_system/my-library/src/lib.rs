mod front_of_house;

pub use crate::front_of_house::hosting;

// re-exporting

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}