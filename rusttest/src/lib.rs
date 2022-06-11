pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn prints_and_returns_100(a: i32) -> i32 {
    println!("i got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    #[ignore]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
    // #[test]
    // fn this_test_will_pass() {
    //     let value = prints_and_returns_100(4);
    //     assert_eq!(10, value);
    // }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_100(8);
    //     assert_eq!(5, value);
    // }
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn it_success() {
    //     assert_ne!(4, 5);
    // }
}