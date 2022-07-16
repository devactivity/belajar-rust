use heycargo::PrimaryColor;
use heycargo::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
    println!("Hello world");
}