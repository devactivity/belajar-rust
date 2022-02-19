// Struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct MyStruct; // unit-like struct => ()

fn main() {
    let mut user1 = User {
        email: String::from("admin@email.com"),
        username: String::from("admin"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user@email.com"),
        username: String::from("user"),
        ..user1
    };

    user1.email = String::from("administrator@email.com");


    println!("Hello: {}", user1.username);

    let black = Color(0, 0, 0);
    let center = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}