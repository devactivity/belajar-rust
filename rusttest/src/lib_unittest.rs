// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// pub fn greeting(name: &str) -> String {
//     String::from("hello")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("RustLang");
//         assert!(
//             result.contains("RustLang"),
//             "Greeting did not contain name, value was {}",
//             result
//         );
//     }

//     // #[test]
//     // fn it_adds_two() {
//     //     assert_ne!(4, add_two(2));

//     //     // assert!(4 == add_two(2));
//     // }
//     // #[test]
//     // fn it_works() {
//     //     let result = 2 + 2;
//     //     assert_eq!(result, 4);
//     // }
//     // #[test]
//     // fn another() {
//     //     panic!("Make this test fail");
//     // }
// }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         // assert!(larger.can_hold(&smaller));
//         assert!(!smaller.can_hold(&larger));
//     }
// }


// pub struct Guess {
// 	value: i32,
// }

// impl Guess {
// 	pub fn new(value: i32) -> Guess {
// 		// if value < 1 || value > 100 {
// 		if value < 1 {
//             panic!("Guess value must be greater than or equal to 1, got {}.", value);
// 		} else if value > 100 {
//             panic!("Guess value must be less than or equal to 100, got {}.", value);
//         }

// 		Guess { value }
// 	}
// }


#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // fn greater_than_100() {
    //     Guess::new(200);
    // }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 4 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 tidak sama dengan 4"))
        }
    }
}