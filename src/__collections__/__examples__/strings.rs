fn main(){
    // String is a wrapper around Vec so it has many of its operations
    let mut s = String::new();

    // to string is available on any type that implements Display trait
    // to_string and String::from createsString from string literals
    let data = "initial contents";
    let s2 = data.to_string();
    let s3 = String::from("initial contents");
    // the method also works on a literal directly:
    let s2 = "initial contents".to_string();

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // push_str take an &str because we want to use string after push so we dont get ownership
    let mut ss = String::from("foo");
    let ss1 = "bar";
    ss.push_str(ss1);
    println!("ss1 is {ss1}");

    // push will add a single character
    let mut s4 = String::from("lo");
    s4.push('l');

    // one way to concatenated strings is this
    let sss = String::from("Hello, ");
    let sss1 = String::from("world!");
    let sss2 = sss + &sss1; // note s1 has been moved here and can no longer be used
    // the + operator has this signature
    // fn add(self, s: &str) -> String; it gets the ownership of the calling string
    // compiler automatically converts &String to &str
    // add finally returns ownership to the result

    // for complicated concatinations use format! because add will be unexpected
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    // rust doesnt allow indexing into strings because they are UTF-8 and may return unexpected values
    // in non ascii characters each character is bigger than 1Byte so indexing into string returns a Byte value and not the character
    let s1 = String::from("hello");
    let h = s1[0];
    let hello = String::from("Hola"); // 4Bytes
    let hello = String::from("Здравствуйте"); // 24Bytes instead of 12Bytes

    // istead of indexing a single byte we use ranges to extract string slice front an string
    // if our range was not a valid character rust will panic like &hello[0..1]
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    // iterate over string characters
    for c in "Зд".chars() {
        println!("{c}");
    }

    // iterate over string bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}