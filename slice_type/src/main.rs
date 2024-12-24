fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);

    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    let s = String::from("hello");

    let _slice = &s[0..2];
    let _slice = &s[..2];

    let s = String::from("hello");

    let len = s.len();

    let _slice = &s[3..len];
    let _slice = &s[3..];

    let s = String::from("hello");

    let len = s.len();

    let _slice = &s[0..len];
    let _slice = &s[..];

    let s = String::from("hello world");

    let word = first_word_v2(&s);

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
