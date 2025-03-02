/*

Given a list of integers, use a vector
and return the median (when sorted, the value in the
middle position) and mode (the value that occurs most often;
a hash map will be helpful here) of the list.

*/

use std::collections::HashMap;

fn median_and_mode(numbers: &mut Vec<i32>) -> (f64, i32) {
    numbers.sort(); //in built function to sort the bumbers vector

    let median = if numbers.len() % 2 == 0 {
        let mid1 = numbers[numbers.len() / 2 - 1] as f64;
        let mid2 = numbers[numbers.len() / 2] as f64;
        (mid1 + mid2) / 2.0
    } else {
        numbers[numbers.len() / 2] as f64
    };

    //lets calculate the value of mode
    let mut count_map_for_mode = HashMap::new();
    for num in numbers.iter() {
        let count = count_map_for_mode.entry(*num).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    let mut mode = numbers[0];

    for (k, v) in count_map_for_mode {
        if v > max_count {
            max_count = v;
            mode = k;
        }
    }
    (median, mode)
}
fn main() {
    let mut numbers = vec![3, 1, 2, 1, 3, 4, 5, 1, 6];
    let (median, mode) = median_and_mode(&mut numbers);
    println!("Median: {:?} and Mode: {:?}", median, mode);
}
