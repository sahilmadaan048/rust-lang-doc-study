fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}"); //this will give error at compile time only
                        //cus this is a dangling reference
}

//main purpose of lifetomes is to avoid dangling references
//these are ref to a memory location which has been cleared out and nothing exists there anymore


/*
unlike traits (which is used to define the behaviour)
lifetimes ensire thet ref aere valid as lonf as we need them to be

every ref in rist has a lifetiome associated with it

we must annotate lifetimes when the lifetimes of references could be related in a few different ways

rust uses a borrow cheker to check whether the given code is ce is valid or invalid\
which compares the scopes to check whether a;; bowrrows are valuid

*/

fn main() {
    let r;                // ---------+-- 'a -> r ka lifetime
                          //          |  
    {                     //          |
        let x = 5;        // -+-- 'b  |  => x ka lifetime
        r = &x;           //  |       |  
    }                     // -+       |   
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+

//this gets rejected at compile time cuz lifetime of x is smaller than the lifetime of r

/*
generic lifetimes in functions


*/

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
//this will give error fn longest(x: &str, y: &str) -> &str {
//   |               ----     ----     ^ expected named lifetime parameter

//since the largest function could be callled from anywhere in the main.rs block,  we need to explivitly tell the compiler
//the lifetimes of the references being passed as arguments in the function call

/*
The borrow checker can’t determine this either, because 
it doesn’t know how the lifetimes of x and y relate to the 
lifetime of the return value. To fix this error, we’ll add 
generic lifetime parameters that define the relationship between the 
references so the borrow checker can perform its analysis.
*/

//LIFETINE ANNOTATION SYNTAX
/*
generic lifetime annotations dont change how long any of the references live
rather  they describe the relationshuop of the lifetimes of multiple
reference to each other without affecring tge lifetiomes
just as functions

*/

/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

// Lifetime Annotations in Function Signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


/*
We want the signature to express the following constraint: 
the returned reference will be valid as long as both the 
parameters are valid.
*/


///here we are not changing some lifetime, we are just specifying yhat the
//borrow checker should reject amy values that dont adhere to these constraints

//these annotations goin the fn definintion and not in the function body

//it also decrease the load on the compiler in the long run
//sinnce the compiler will have less inference sgo makea bout the scope of the parameters passed in the function call

//the lifetime for the returned value is tje smaller of the lifetimes if the input parameters

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}"); //will not compile since the scope of result is the inner vlock where string2 was decllared and initialised
}

// Thinking in Terms of Lifetimes
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}


fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}//compilation fails here because even if thelifteimme is defined for the reurned value
//it is not related to the lifeuime of the parameters at all
//and  in this case it is trying to create a dangling reference which rust would never allow


/*
the lifetime of the returned value must  matcj with the lifteime of one of the vapssed values

*/
//the static lifetoime
let s: &'static str=  "i have a static lifetime";

// The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.

//Generic Type Parameters, Trait Bounds, and Lifetimes Together

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*

This is the longest function from Listing 10-21 that returns 
the longer of two string slices. But now it has an extra parameter 
named ann of the generic type T, which can be filled in by any type 
that implements the Display trait as specified by the where clause. 
This extra parameter will be printed using {}, which is why the Display 
trait bound is necessary. Because lifetimes are a type of generic, 
the declarations of the lifetime parameter 'a and the generic type parameter 
T go in the same list inside the angle brackets after the function name.


*/


// Lifetime Annotations in Struct Definitions

struct ImportantExcerpt<'a> {
    part: &'a str,//string slice = refernce
}
//this means that an instance of struct cant outlive the reference of it holds in the part fuiels


fn main() {
    let novel = String::from("Call me Ishmael. Some years ago..."); //owns the first sentence originally
    let first_sentence = novel.split('.').next().unwrap();
    //it has now a refernce to tgee first sentence
    let i = ImportantExcerpt {
        part: first_sentence,
    };//this instamnce of struct is valid for this lifetime since the lifetime of the actual owner which is novel is through the entire main function here

}

//LIFETIME ELISIOJ RULES
/*
set of rules followed by the compiler based on some detrerminsitic patterns of function signatures
thus we dont have to write the lifetime annotations explicitly every time
*/

//onw such example s this where it compiles sucessfully even without lifetime annotattions

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//with lifetime annotations , the function signature wiuld look something like ths
fn first_word<'a>(s: &'a str) -> &'a str {}

/*
there are 3 rules eehich the como=iler follows to check for lifetimes when they are not explicitly deinfed by the develeoper
*/

/**
 *  1.The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

    2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

    3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
 */
fn first_word(s: &str) -> &str {}
fn first_word<'a>(s: &'a str) -> &str {}
fn first_word<'a>(s: &'a str) -> &'a str {}


fn longest(x: &str, y: &str) -> &str {}
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
//it still couldnt figure out the lifetome for the return value so
//compiler wil lrespond with an error


// Lifetime Annotations in Method Definitions

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
//lifetime elision rules have been applied to them
//see imp/image.png for this


/*
Lifetimes on function or method parameters are called input lifetimes, 
and lifetimes on return values are called output lifetimes.
*/

