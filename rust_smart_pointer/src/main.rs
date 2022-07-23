enum List {
    Cons(i32, Box<List>),
    Nil,
}

// enum Message {
//     Quit,
//     Move {x:i32, y:i32},
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }

use crate::List::{Cons, Nil};

fn main() {
    // let b = Box::new(5); <= unnecessary
    // println!("b = {}", b);
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
// Box<T>
// recursive type
// cons list (1, (2, (3, Nil)))