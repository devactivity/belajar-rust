// Default Generic Type Parameters and Operator Overloading
// ================================================================
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// trait Add<Rhs=Self> { // right hand side
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

fn main() {
    assert_eq!(
        Point {x: 1, y:0} + Point {x:2, y:3},
        Point {x: 3, y:3}
    );
}



// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
// ================================================================
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is you captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up.");
    }
}

impl Human {
    fn fly(&self) {
        println!("Waving arms furiously");
    }
}

fn main() {
    let person = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // or Human::fly(&person)
}


trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // fully qualified syntax
}




// Using Supertraits to Require One Trait’s Functionality Within Another Trait
// ================================================================
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point {x: 1, y: 3};
    p.outline_print();
}


// Using the Newtype Pattern to Implement External Traits on External Types
// ================================================================
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(
        vec![
            String::from("hello"),
            String::from("world"),
        ]
    );

    println!("w = {}", w);
}