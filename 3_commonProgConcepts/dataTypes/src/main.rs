fn main() {
    println!("Hello, world!");

    let guess: u32 = "442".parse().expect("not a number: ");

    println!("{guess}");

    let mut a: u8 = 10;
    println!("{:?}", a);

    // a = 300; //will give error
    //cargo run --release does the check for overflow errors rather i

    //performs twos c/omoplement wrapping 257 becomes 0 like that in circuar way
    println!("{:?}", a);

    //floatinf value es
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    //numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    //boolean type
    let _t = true;

    let _f: bool = false; // with explicit type annotation

    //character data type
    let c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound data types
    //Compound types can group multiple values
    // into one type. Rust has two primitive compound types:
    // tuples and arrays.
    //

    // tuples => cannot grow or shrink unlike vectors in c++

    let tup: (i32, f64, u8) = (500, 9.0, 1);

    let tup2 = (500, 6.4, 1);

    let (x, y, z) = tup2; //pattern matching to get the vlaues from the tuple
    //this process is called destructuring

    println!("The value of y is: {y}");

    let _x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = _x.0;
    let six_point_four = _x.1;

    let one = _x.2;

    println!("value of one is: {one}");

    /*
    The tuple without any values has a special name, unit.
     This value and its corresponding type are both written
     () and represent an empty value or an empty return type.\
     xpressions implicitly return the unit value if they don’t return
     any other value.

     */

    //arrays un rust
    let arr = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let temp_arr: [i32; 5] = [1, 2, 3, 4, 5];
    let temp2 = [3; 5]; //means this [3, 3, 3, 3, 3]

    let first = temp2[0];
    let second = temp2[1];

    println!("first is {first} and second is: {second}");
}
/*
data type tells which kind pf data is benig stored by te variable
scalar and comppund data types



scalar sata typesare of 4 types
integer, floating, characters, booleans


So how do you know which type of integer to use? If you’re unsure,
Rust’s defaults are generally good places to start: integer types default
to i32. The primary situation in which you’d use
 isize or usize is when indexing some sort of collection.
 */
