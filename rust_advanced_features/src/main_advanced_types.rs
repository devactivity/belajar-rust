// Using the Newtype Pattern for Type Safety and Abstraction
// =========================================================
use rust_advanced_features::People;

fn main() {
    let mut p = People::new();
    p.add_name("RustLang");

    println!("{:?}", p);
}

// Creating Type Synonyms with Type Aliases
// =========================================================
fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type MyType = Box<dyn Fn() + Send + 'static>;

    let f: MyType  = Box::new(|| println!("tes"));

    fn takes_long_type(f: MyType) {}

    fn returns_long_type() -> MyType {
        Box::new(|| ())
    }


    use std::fmt;

    type Result<T> = std::result::Result<T, std::io::Error>;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }


}

// The Never Type that Never Returns
// =========================================================
fn bar() -> ! { // diverging function
    panic!();
}

enum Option<T> {
    Some(T),
    None,
}

use crate::Option::*;

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("calledd Option::unwrap() on a None value")
        }
    }
}

fn main() {
    print!("forever");
    loop {
        print!("infinite");
        break;
    }
}

// Dynamically Sized Types and the Sized Trait
// =========================================================
fn main() {
    let s1: &str = "hello world";
    let s2: &str = "hello rust";

    fn generic<T>(t: T) {} // from this
    fn generic<T: ?Sized>(t: &T) {} // to this (implicit in Rust)
}