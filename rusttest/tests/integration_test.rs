use rusttest;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rusttest::add_two(2));
}