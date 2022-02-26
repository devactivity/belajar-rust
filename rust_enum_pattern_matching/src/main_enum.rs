// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

// prelude
// enum Option<T> {
//     None,
//     Some(T),
// }

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    // let some_number = Option::Some(5);
    // let some_str = Some("hello");
    // let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home4 = IpAddrNew::V4(127, 0, 0, 1);
    // let home6 = IpAddrNew::V6(String::from("::1"));

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    let msg = Message::Write(String::from("hello"));
    msg.call();

    println!("Enums and Pattern Matching");
}
