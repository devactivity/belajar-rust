use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// contoh deref coercion
fn hello(name: &str) {
    println!("Hello, {}", name);
}

impl<T> Deref for MyBox<T> {
    type Target = T; // associated type

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// DerefMut
// Drop Trait (drop)

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // let x = 5;
    // // let y = &x;
    // // let y = Box::new(x);
    // let y = MyBox::new(x);

    // let m = MyBox::new(String::from("Rust Dev"));
    // hello(&m);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);  // *(y.deref())

    // drop trait example
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };

    println!("CustomSmartPointer created.");
    // c.drop(); // std::mem::drop
    drop(c);
    println!("CustomSmartPointer dropped before the of the main scope.");

}