use aggregator::{Summary, Tweet, NewsArticle, notify, Pair};

fn main() {
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Lorem ipsum lorem ipsum"),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Cara baru membuat tweet"),
        location: String::from("Los Angeles"),
        author: String::from("rust_lang"),
        content: String::from("Lorem ipsum content"),
    };
    // println!("New article available: {}", article.summarize())

    // println!("{:?}", notify(&tweet));

    let p = Pair::new(2, 6);
    println!("{:?}", p.cmp_display());

    let s = 3.to_string();


}

