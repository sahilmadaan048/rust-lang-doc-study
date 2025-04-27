/*
Patterns ans matching


gives more ontrol over programs control flow

pattern can be a combination of following
    -> literaals
    -> destructured arrays, enums, structs and tuples
    -> variables
    -> wildcrad
    ->> placehilders

*/

/*
to use a pattern , we compare it pto some value => if the pattern matches the valeue, we use the values as parts oin opoir code

*/


/*
refutable and irrefutable patterns, pattern syntax that you mught see


*/

match VALUE {
    PATTERN1 => EXPRESSION1,
    PATTERN2 => EXPRESSION2,
    PATTERN3 => EXPRESSION3,
}

//match expresiion that we have been using up until now
match x {
    None => None,
    Some(i) => Some(i + 1),
}
//match expressions need to be exhaustive
//other covers all the remaining cases but it binds with thw value also, so we have to use at the end of program
// _ placeolder does the same thing but does not bind to the value of the variable
//thus it is preferred and can be used anywhere in the code


/*
considional if let expressions

it is inda lie a hybrid between pattern matching usinf match expression and
if else control flow contruct which wr generallyu see


if is not explitlt exhaustuve, we can mae it if we want to
*/


//example of of let
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();  //parse it to get u8

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { //i=we cant combine the age in ok varinat and the age condition age > 30 in the same line
        if age > 30 { 
            //this age becomes valid to be sused onlt after it is defined in the if let expression the 2 lines aBove
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}


/*
While let conditional loop

lets us run a loop for as long as a pattern continues to match

read through chapter 17 to do this stream thingy and thread spawin in parallel

*/

let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for val in [1, 2, 3] {
        tx.send(val).unwrap();
    }
});

while let Ok(value) = rx.recv() {  //retuns an Result enum Result<value, Steing> {
    // Ok (value),  Err(msg) => println!(msg.to_string()) } 
    println!("{value}");;
}



/*
for loops
*/

let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{value} is at index {index}");
}

let PATTERN = EXPRESSION;  //always whenever we have used let kwyword that was pattern matching

let x = 5;  //x is a pattern that means vinad what mathes here to the varieble x

//because of the name x is the whole pattern, this pattern effectively means bind everything to the vatiable x whatever the value is

let (x, y, z) = (1, 2, 3);  //here compiler sees => pattern is matching

let (x, y) = (1, 2, 3);  //since the number os variables on lhs is lesser than on rhs => pattern is not matching this compile error comes her
 

/*
FUNCTIOON PARAMETERS

can also be patterns

*/

fn foo(x: i32) {
    // code goes here
}

//here x part is the pattern

//as using let we can also do the tuple thingy here
//for pattern matching
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

//next we will learn about refutable and irredutable patterns