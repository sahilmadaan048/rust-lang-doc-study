fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s2 = String::from("sahil madaan");
    change(&mut s2); //able to use it here again as a referenve could not have been possible if i would have passed the string s1
    //into the calculate_length function as move without creating a refernve

    println!("{}", s2);

    let mut s = String::from("dahi jamaa lo");

    // let r1 = &mut s;
    let r2 = &mut s;

    // println!("{}, {}", r1, r2);
    println!("{}", r2);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    //gives ni error means it is allowed
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
a refernce is like a pointer in that its an address we can follow
to access the data stored at that address
that data  is owned by some other variable`
*/

/*
The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it
*/

fn change(some_string: &mut String) {
    some_string.push_str(", is my name");
}

/*
the concept tof mutable refernces comes in

just as variables are immutable by defsult in rist so are referenve \
we cnnot make changes in them wothout usong thr leyword mut wheerever that referbce is used as a paremater
whether it be function callin gor funcriondefinitiomn
or the string declaration
 */

/*
restriction if using references in rust is we can only create one refernce to a variable at a time

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.
The benefit of having this restriction is that Rust can prevent data races at compile time.

it is a dat arace if two folllowing 3 conditions hold
    1. there are more than 1 pointer trying to access the data at atime
    2. and atleast one of the ppin ter is trying tp write the data to that location
    3. there's no mechanim being used to synchronise accesss to the data

data races are undefined behaviour that can be difficult to track down and deal with aor diagnose at the runtime
'

 */

/*
rust allows us to define multiple refernces to the same variable but not the simultaneous ones
using curcly brackets to crear=te a new scope and do your thing
 */


/*

somthing similar is the case with using mutable and immutable references together

we cannot make any mutable refernce before the completion of use of any immutable 
reference defned on that variable

xus we cant allow the user to write the data when hopefully someone else is reading the data
since ther is no way to synchronise the access if the data in case he/she changes the data
leading to undefined beghavior
 */


//this is the reason following code gives error
//
//    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);




//while the following code does not give any error


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");



/*
thus the authority to define a mutable refernce to a variable depends on the scope of the immutable
reference used in the code
 */


/*

DAANGLING REFERENCES
m
if we free up some memory while preserving a pointer to that memory
noe this pointer references to a memory location which may have 
allocated to someone else

this is called a dngling pointer 

 */


// this function's return type contains a borrowed value, but there is no value
// for it to be borrowed from



fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

//solution to this is to do the following

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}


//this wors out perfectly, without any problems since the ownership of string is moved out and this nothing is deallocated


/*

The Rules of References
Let’s recap what we’ve discussed about references:

At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.

Next, we’ll look at a different kind of reference: slices.


 */