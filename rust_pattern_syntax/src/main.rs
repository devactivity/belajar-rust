// ### Matching Literals
// =======================================================
fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}



// ### Matching Named Variables
// =======================================================
fn main() {
    let x =None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}



// ### Multiple Patterns
// =======================================================
fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}



// ### Matching Ranges of Values with ..=
// =======================================================
fn main() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        // 1 | 2 | 3 | 4 | 5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn main() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        // 1 | 2 | 3 | 4 | 5 => println!("one through five"),
        _ => println!("something else"),
    }
}



// ### Destructuring to Break Apart Values
// =======================================================
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    // let Point { x: a, y: b } = p;
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x at {}", x),
        Point { x: 0, y } => println!("On the y at {}", y),
        Point { x, y } => println!("On neither at {}, {}", x, y),

    }
}



// #### Destructuring Enums
// =======================================================
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}



// #### Destructuring Nested Structs and Enums
// =======================================================
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}



// #### Destructuring Structs and Tuples
// =======================================================
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let (
        (feet, inches),
        Point { x, y }
    ) = ((3, 10), Point { x: 3, y: -10 });
}



// #### Ignoring an Entire Value with _
// =======================================================
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}



// #### Ignoring Parts of a Value with a Nested _
// =======================================================
fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn main() {
    let numbers = (1,2,3,4,5);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}



// #### Ignoring an Unused Variable by Starting Its Name with _
// =======================================================
fn main() {
    // let _x = 5;
    // let y = 10;

    let s = Some(String::from("hello"));

    if let Some(_) = s {
        println!("Found a string")
    }

    println!("{:?}", s);
}



// #### Ignoring Remaining Parts of a Value with ..
// =======================================================
fn main() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2,4,8,16,32);

    match numbers {
        (first, .., last) => {
            println!("SOme numbers: {first}, {last}")
        }
    }
}



// #### Extra Conditionals with Match Guards
// =======================================================
fn main() {
    let num = Some(3);

    match num {
        Some(x) if x > 10 => println!("Lebih besar dari 10, x = {}", x),
        Some(x) => println!("Lebih kecil dari 10, x = {}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no")
    }
}


// #### @ Bindings
// =======================================================
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}