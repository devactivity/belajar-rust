mod my_module {
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("add to waitlist");
            }
        }
    }

    pub use front_of_house::hosting;

    pub fn eat_at_restaurant() {
        println!("eat at restaurant");
        hosting::add_to_waitlist();
    }
}

use my_module::eat_at_restaurant;
mod foo; // <- module declaration
use foo::item;




// use std::{cmp::Ordering, io};
// use std::io;
// use std::io::Write;

// use std::io::{self, Write};

use std::collections::*;



fn main() {
    item();
    eat_at_restaurant();
    println!("in main");
    my_module::hosting::add_to_waitlist();
}