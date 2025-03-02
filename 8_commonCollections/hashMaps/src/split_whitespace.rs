fn split_whitespace_custom(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = None;
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if !c.is_whitespace() {
            if start.is_none() {
                start = Some(i);
            }
        } else {
            if let Some(st) = start {
                result.push(&s[st..i]);
                start = None;
            }
        }
    }

    if let Some(st) = start {
        result.push(&s[st..]);
    }

    result
}

fn main() {
    let text = "Hello   world! This is Rust.";

    // Using custom split_whitespace function
    for word in split_whitespace_custom(text) {
        println!("{}", word);
    }
}
