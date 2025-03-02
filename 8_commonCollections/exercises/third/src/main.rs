/*

Using a hash map and vectors,
create a text interface to allow a user to add
employee names to a department in a company;
for example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department
or all people in the company by department, sorted alphabetically.

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
        Some(c) if vowels.contains(&c.to_string().as_str()) => format!("{}-hay", word),
        Some(c) => format!("{}-{}ay", &word[1..], c),
        None => String::new(),
    }
}

fn manage_company() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    company
        .entry("Engineering".to_string())
        .or_insert(vec!["Sally".to_string()]);
    company
        .entry("Sales".to_string())
        .or_insert(vec!["Amir".to_string()]);

    for (department, employees) in &mut company {
        employees.sort();
        println!("{}: {:?}", department, employees);
    }
}

fn main() {
    let mut numbers = vec![3, 1, 2, 3, 4, 3, 5, 1, 6];
    let (median, mode) = median_and_mode(&mut numbers);
    println!("Median: {:.1}, Mode: {}", median, mode);

    let words = vec!["first", "apple", "banana", "orange", "grape"];
    for word in words {
        println!("{} -> {}", word, to_pig_latin(word));
    }

    manage_company();
}
