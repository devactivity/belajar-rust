fn main() {
    let number_list = vec![34, 50, 250, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is: {}", result);

    let number_list = vec![102, 34, 6000, 98, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is: {}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    // `&number` pattern matching, destructuring in Chapter 18
    for &number in list {
        if number > largest {
            largest = number; // replace the number to the higher number
        }
    }
    largest
}