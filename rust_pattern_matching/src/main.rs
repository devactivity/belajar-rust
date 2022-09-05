fn main() {
    println!("Patterns & Matching");

    // let (a, b) = [1, 2];
    // let (a, b) = "Rust";
    // let (a, b) = (10, 20);
}

// - Literals
// - Destructured arrays, enums, structs, tuples
// - Variables
// - Wildcards
// - Placeholders

// match
// =======================================================
#[derive(Debug)]
enum Asia {
    Indonesia,
    Malaysia,
    Singapura,
}

fn main() {
    let country = Asia::Indonesia;

    match country {
        Asia::Indonesia => println!("Selamat Hari Raya"),
        Asia::Malaysia => println!("Selamat Hari Raye"),
        // _ => println!("Other country"),
        c => println!("Other country {:?}", c)
    }
}


// if let
// =======================================================
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("User your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Useing purple as the background color");
        } else {
            println!("Useing orange as the background color");
        }
    } else {
        println!("Useing blue as the background color");
    }
}


// while let
// =======================================================
fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}


// for loop
// =======================================================
fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}


// let statement
// =======================================================
fn main() {
    let x = 5;
    // let PATTERN = EXPRESSION;

    let (x, y, z) = (1, 2, 3);
}


// function parameter
// =======================================================
fn main() {
    let point = (3, 10);
    print_coordinates(&point);

    let x = 5;

    let some_option_value: Option<i32> = None;
    if let Some(x) = some_option_value {
        println!("{}", x);
    };

    if let q = 5 {
        println!("{}", q);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
