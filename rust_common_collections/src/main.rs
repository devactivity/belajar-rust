fn main() {
    // println!("hashmap");
    // HashMap<K, V>
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Napoli"), 2);
    scores.entry(String::from("Lazio")).or_insert(5);

    scores.entry(String::from("Napoli")).or_insert(3);
    println!("{:?}", scores);
    // scores.insert(String::from("Lazio"), 1);

    // let team_name = String::from("Napoli");
    // let score = scores.get(&team_name);
    // println!("{:?}", score);

    // for (key, value) in &scores {
    //     println!("{}:  {}", key, value);
    // }

    // let the_question = String::from("my favorite color");
    // let the_color = String::from("white");

    // let mut map = HashMap::new();
    // map.insert(the_question, the_color);
    // println!("{}", the_color);

    let text = "hello world wonderful world";
    let mut data = HashMap::new();

    for word in text.split_whitespace() {
        let count = data.entry(word).or_insert(0); // &mut V
        *count += 1
    }

    println!("{:?}", data);

}