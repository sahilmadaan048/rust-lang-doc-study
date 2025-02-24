/*

Generate the nth Fibonacci number.


*/

use std::io;

fn main() {
    println!("Enter the value of n: ");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Enter a valid number n! ");

    let n: i32 = n.trim().parse().expect("Enter a valid number n! ");

    //to print thw nth fibonacci using simple bvariables
    //we will leep two variables prev1 and prev2 and loop till n
    let mut prev1 = 1;
    let mut prev2 = 0;

    if n == 1 {
        println!("{prev2}");
        return;
    };
    if n == 2 {
        println!("{prev1}");
        return;
    };

    for _ in 2..n {
        let fib = prev1 + prev2;
        prev2 = prev1;
        prev1 = fib;
    }
    println!("the {n}th fibonacci number is: {prev1} ");
}
