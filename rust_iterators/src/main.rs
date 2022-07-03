fn main() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("element: {}", val);
    // }


    // pub trait Iterator {
    //     type Item;

    //     fn next(&mut self) -> Option<Self::Item>;
    //     // [...]
    // }
}
