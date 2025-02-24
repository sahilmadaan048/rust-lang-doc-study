fn main() {
    let s = String::from("hello");

    takes_ownership(s);
    //since now its ownership has been passed to the funtion, we can no longer use it in the main funcion
    let x = 48;

    makes_copy(x);

    println!("{}", x);

    let s1 = gives_ownership(); // gives ownership moves its return value intp s1
    let s2 = String::from("hellp"); //s2 comes into scope
    let s3 = takes_and_gives_back(s2); //s2 is moved into
    //this fn which also moves its return value into s3
    // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    //deep copy generated thus it can be used here as it is
} //here x goes out of scope then s, but since s value was moved durong the function call 
//nothing special happens

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} //here, some_string goes out of scope, and drop is called the backing
//memory is freed by the RUST compiler and returned back to the memory colector

fn makes_copy(some_integer: u32) {
    println!("{}", some_integer);
} //here, some_integer goes out of scope, nothing special happens
//since it was stack memory only, the concept of move or does not happen here

/*
passing a variable to a function will move or copy, just as assignment does

 */

/*
returning vales can alsom transfer ownerships

 */

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

/*
what if we want to let a function ise a value and not take ownership

method 1 as we did before is giving and takning back ownersho[

method 2 is using refernces for doing this

 */
//-----------example of rust returning a tuple

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

//usize dattatype basically means unsiggned size of the system architecture depending on the machine
////whether 32 bit or 64 bit it may vary accordingly

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
