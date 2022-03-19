fn main() {
    // Vec<T>
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);

    {
        let v2 = vec![1,2,3,4,5];
        // v2 valid inside here
    } // <- v2 is dropped here

    let v3 = vec![1,2,3,4,5];
    let third = &v3[2];

    match v3.get(2) {
        Some(val) => println!("element found: {}", val),
        None => println!("element not found")
    }

    println!("{:?}", third);

    let v4 = vec![1,2,3,4,5];
    // let not_found_index = &v4[50];
    let not_found_index = v4.get(50);
    println!("{:?}", not_found_index);

    // let mut v5 = vec![1,2,3,4,5];
    // let first = &v5[0];

    // v5.push(6);
    // println!("{}", first);
    let v6 = vec![1,2,3];
    // for i in &v6 {
    //     println!("{}", i);
    // }

    let mut v7 = vec![1,2,3];
    for i in &mut v7 {
        *i += 10; // * dereference chapter 15
    }
    println!("{:?}", v7);

    enum Excel {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Excel::Int(5),
        Excel::Text(String::from("hello")),
        Excel::Float(1.2)
    ];

}
