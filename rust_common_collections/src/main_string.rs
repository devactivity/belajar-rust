fn main() {
    // let mut s = String::from("initial data");
    // let data = "initial data".to_string();

    // let hello = String::from("السلام عليكم"); // arabic
    // let hello = String::from("नमस्ते"); // hindi
    // let hello = String::from("こんにちは"); //japan
    // let hello = String::from("안녕하세요"); // korea
    // let hello = String::from("你好"); // mandarin
    // let hello = String::from("Здравствуйте"); //russia

    // let mut s = String::from("foo");
    // s.push_str("bar");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("{}", s2);

    // let mut s2 = String::from("lo");
    // s2.push('l');


    // let s1 = String::from("hello");
    // let s2 = String::from("world");
    // // let ss = &s2;
    // let s3 = s1 + &s2; // &s2 -> &s2[..], add(), defer coercion (chapter 15)
    // println!("{}", s3);

    // fn add(self, s: &str) -> String {}
    // let s1 = String::from("from");
    // let s2 = String::from("new");
    // let s3 = String::from("world");

    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", s);

    // let hello = String::from("Hola");
    // // String -> Vec<u8>
    // // let h = s1[0];

    // let hello = String::from("Здравствуйте"); //russia

    // bytes, unicode scalar values (char), grapheme cluster (huruf)

    // let s = String::from("नमस्ते");

    // // Vec<u8>
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    // // 224, 165, 135]

    // // unicode scalar value
    // ['न', 'म', 'स', '्', 'त', 'े'] // diacritics

    // // grapheme cluster
    // ["न", "म", "स्", "ते"]


    let hello = String::from("Здравствуйте"); //russia
    // let hello = String::from("hola"); //russia
    let s = &hello[0..=3];
    println!("{}", s);

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }

}
