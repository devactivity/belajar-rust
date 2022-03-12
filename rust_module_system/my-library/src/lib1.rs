mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        // fn seat_at_table() {}
    }

    mod serving {
        // fn take_order() {}

        // fn serve_order() {}
        // mod back_of_house {
        //     fn fix_incorrect_order() {
        //         cook_order();
        //         super::serve_order();
        //     }
        //     fn cook_order() {}
        // }

        // fn take_payment() {}
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn eat(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("apple") }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}


// use crate::front_of_house::hosting;
use self::front_of_house::hosting;

// ../music
pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut meal = front_of_house::Breakfast::eat("sari roti");
    meal.toast = String::from("borobudur");
    println!("{}", meal.toast);
    // meal.seasonal_fruit = String::from("orange");

    // let order1 = front_of_house::Appetizer::Soup;
    // let order2 = front_of_house::Appetizer::Salad;
}