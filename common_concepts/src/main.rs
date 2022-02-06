fn main() {
    // Variables
    // ---------------------------------------
    // let a = 10; //immutatable
    // let mut b = 20; // mutable

    // const HIGHEST_PRICE: u32 = 10_000_000;

    // let x = 10;
    // let x = "ten";

    // Data Types
    // ---------------------------------------
    // scalar: integer, floating number, boolean, characters
    // let y: u16 = 10;
    // let f: f32 = 2.1;
    // let valid = true;
    // let invalid: bool = false;
    // let c = 'Z';

    // compound: tuples dan arrays
    // let tup: (i32, f64, u8) = (100, 1.3, 1);
    // let (x, y, z) = tup;
    // let first = tup.0;

    // let tup2 = ();

    // let a = [1,2,3];
    // let b: [i32; 4] = [1,2,3,4];
    // let b = [3, 5]; // => let b = [3,3,3,3,3];
    // let first = b[0];

    // println!("hello");
    // my_function(3, 'h');
    // let res = valid_function(1,2);
    // println!("{}", res);

    // Control Flow
    // ---------------------------------------
    // let num = 6;

    // if num < 5 {
    //     println!("condition true");
    // } else if num % 3 == 0 {
    //     println!("num is divisible by 3");
    // } else {
    //     println!("condition false");
    // }

    // let cond = true;
    // let num2 = if cond { 5 } else { 6 };

    // loop {
    //     println!("looping...");
    // }
    // let mut counter = 0;
    // let res = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2
    //     }
    // };
    // println!("{}", res);

    // let mut num = 3;

    // while num != 0 {
    //     println!("{}", num);
    //     num -= 1;
    // }
    // println!("bye..");

    // let a = [1,2,3];

    // for element in a {
    //     println!("the value is : {}", element);
    // }

    for number in 1..=5 {
        println!("{}", number);
    }
    println!("bye..");
}


// Functions
// ---------------------------------------
// fn my_function(value: i32, label: char) {
//     println!("Result {}{}", value, label);
// }

// fn valid_function(x: i32, y: i32) -> i32 {
//     x + y
// }

// Comments
// ---------------------------------------
// inline comment here
/*
    block comment
*/
