/*

Convert strings to pig latin. The first consonant of
each word is moved to the end of the word and ay is added,
so first becomes irst-fay. Words that start with a vowel have hay
added to the end instead (apple becomes apple-hay). Keep in mind the details
about UTF-8 encoding!

 */

use std::collections::HashMap;

fn median_and_mode(numbers: &mut Vec<i32>) -> (f64, i32) {
    numbers.sort();

    let median = if numbers.len() % 2 == 0 {
        let mid1 = numbers[numbers.len() / 2 - 1] as f64;
        let mid2 = numbers[numbers.len() / 2] as f64;
        (mid1 + mid2) / 2.0
    } else {
        numbers[numbers.len() / 2] as f64
    };

    let mut occurrences = HashMap::new();
    for num in &*numbers {
        let count = occurrences.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut mode = numbers[0];
    for (&num, &count) in occurrences.iter() {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }

    (median, mode)
}

fn to_pig_latin(word: &str) -> String {
    let vowels = ["a", "e", "i", "o", "u"];
    let first_char = word.chars().next();

    match first_char {
        //fisrtconvert the string slice into a String
        Some(c) if vowels.contains(&c.to_string().as_str()) => format!("{}-hay", word),

        Some(c) => format!("{}-{}ay", &word[1..], c),
        None => String::new(),
    }
}

fn to_pig_latin2(word: &str) -> String {
    let vowels = "aeiou";
    match word.chars().next() {
        Some(c) if vowels.contains(c) => format!("{}-hay", word),
        Some(c) => format!("{}-{}ay", &word[1..], c),
        None => String::new(),
    }
}

fn main() {
    let mut numbers = vec![3, 1, 2, 3, 4, 3, 5, 1, 6];
    let (median, mode) = median_and_mode(&mut numbers);
    println!("Median: {:.1}, Mode: {}", median, mode);

    let words = vec!["first", "apple", "banana", "orange", "grape"];
    for word in words.iter() {
        println!("{} -> {}", word, to_pig_latin(word));
    }
    for word in words.iter() {
        println!("{} -> {}", word, to_pig_latin2(word));
    }
}
