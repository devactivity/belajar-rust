use our_library::Greeting;
use macro_derive::Greeting;
use macro_attribute_like::function_attr;
use macro_function_like::function_like;

#[derive(Greeting)]
struct Alien;

#[function_attr(GET, "/")]
fn _index() {}

fn main() {
    Alien::speak();
    assert_eq!(function_like!(), 6);
    // println!("
    //   Procedural Macro
    //     - custom derive
    //     - attribute-like macro
    //     - function-like macro
    //   ( proc-macro )
    // ");
}


