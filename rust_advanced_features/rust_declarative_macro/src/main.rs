macro_rules! print_tiga {
    ($a:expr, $b:expr, $c:expr) => (println!("{}, {}, and {}", $a, $b, $c);)
}

macro_rules! spy_name {
    ($name:literal) => (println!("My name is{}", $name));
    ($name:literal, $surname:literal) => (println!("My name is {bond}, {james} {bond}", bond=$surname, james=$name));
}
fn main() {
    print_tiga!('a','b', 'c');
    spy_name!("rust");
    spy_name!("rust", "lang");

    println!("
    - Declarative Macro
    - Procedural Macro
        a. Custom #[derive]
        b. Attribute-like macro
        c. Function-like macro

    [metaprogramming, macro_rules!]
    ");

    macro_rules! name {
        (pattern) => {
            expander
        };
    }

    macro_rules! my_macro {
        ($arg:expr) => (println!("arg is {}", $arg));
    }

    let _v: Vec<u32> = vec![1, 2, 3];

    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
}
