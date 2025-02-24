fn main() {
    let mut s = String::from("hello world");

    let _word = first_word(&s); // word will get the value 5

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!

    println!("{_word}");
    s.clear(); // this empties the String, making it equal to ""

    let s1 = String::from("sahil madaan");

    let len = s1.len();
    let hello = &s1[0..5];
    let hello = &s1[0..5];

    let slice = &s1[0..len];
    let slice = &s1[0..]; //this works also
}

//returns the idnex
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes(); //gives a sequance of bytes

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             //byte of space  = b' '
//             return i;
//         }
//     }
//     s.len()
// }

//instead of index, lets actully return the slice ofn t he inout string
// fn first_word(s: &String) -> &str {
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); //gives a sequance of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //byte of space  = b' '
            return &s[0..i];
        }
    }
    &s[..]
}

//we cam also use string slices here  inside or it cpuld be a string literal as well
//
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); //gives a sequance of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //byte of space  = b' '
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}



// can be used on arrays too
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
