fn main() {
    // let x = 5;
    // let y = x; // copy <= Copy Trait (i32, i32)
    // println!("x = {}, y = {}", x, y);

    // let s1 = String::from("hello");
    // let s2 = s1; // moved
    // // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of {} is {}", s1, len);


    // let mut s = String::from("hello");
    // change(&mut s); // create a mutable reference

    // let a = String::from("hello");
    // let r1 = &a;
    // let r2 = &a;
    // println!("{}, {}", r1, r2);

    // let mut b = String::from("hello");
    // let x1 = &b;
    // let x2 = &b;
    // // let x3 = &mut b;
    // println!("{}, {}", x1, x2);

    // let mut p = String::from("hello");
    // let v1 = &p;
    // let v2 = &p;
    // println!("{}, {}", v1, v2);

    // let v3 = &mut p;
    // println!("{}", v3);

    // let reference_to_dangle = dangle();

    let mut sample = String::from("hello world");
    let sample2 = "hello world";
    let word = first_word(&sample);
    // let hello = &sample[..=5];
    // let world = &sample[6..];

    // sample.clear(); // empties the String == ""

    // word still has the value 5 here

    println!("{}", word);

}

// hello world
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s is dropped

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}

// fn gives_ownership() -> String {
//     let some_str = String::from("world");
//     some_str
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// } // here, some_string goes out of scope, drop is called

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }
