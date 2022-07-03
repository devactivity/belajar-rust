#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let in_my_size = shoe_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
        // println!("{:?}", v1_iter);

        assert_eq!(total, 6);
    }

    // #[test]
    // fn iterator_demo() {
    //     let v1 = vec![1, 2, 3];
    //     let mut v1_iter = v1.iter();
    //     // let mut v1_iter = v1.into_iter(); // iter_mut()
    //     // .sum()

    //     assert_eq!(v1_iter.next(), Some(&1));
    //     assert_eq!(v1_iter.next(), Some(&2));
    //     assert_eq!(v1_iter.next(), Some(&3));
    //     assert_eq!(v1_iter.next(), None);
    // }
}