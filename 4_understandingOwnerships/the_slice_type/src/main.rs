fn main() {
    let mut s = String::from("hello world");

    s.clear(); // this empties the String, making it equal to ""
    let _word = first_word(&s); // word will get the value 5

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!

    println!("{_word}");
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
