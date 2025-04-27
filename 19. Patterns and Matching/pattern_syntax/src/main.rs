/*
PATTERN SYNTAX

why and when to use each pattern matching syntax sdiscussed si far

*/

/*
MATCHCING LITERALS
*/


let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}

//used when we want the code to take action when if it gets a particular concrete value

/*
MATCHING NAMEd VARIABLES
*/

let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),  //not this
    Some(y) => println!("Matched, y = {y}"), //yes here this y has a new scope which will stire value = 5
    _ => println!("Default case, x = {x:?}"),  //when x is none
}

println!("at the end: x = {x:?}, y = {y}");


//MULTIPLE PATTERNS
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}


//MATCHING TANGES OF VALUES WITH ..=
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}

//using chars
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}


//DESTRUCTIRING TO BREAK APRT VALUES
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

//shorthand to do the dame thing
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),  //hen y = 0 and x can be anything
        Point { x: 0, y } => println!("On the y axis at {y}"),  //when x = 0 and y can be anything
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }

        //the checkk is done only once in the match expression and it is until the first matching arm is found
    }
}

//DESTRUCTURING ENUMS
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }
}

//DESTTRUCTING NESTED STRUCTS AND ENUMS
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}

//DESTRUCTURING ENUMS AND STRUCTS
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });


//ignoring values in a pattern using _
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
}

//ignoring the parts of a value with a nested _ 
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {setting_value:?}");

//same thing here second and 4th ignored
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {first}, {third}, {fifth}")
    }
}

//ignoring an unused variable by staring its name with _
//while _ doex not bind to the value at all
//_x will get bind to a value ut it will still be ignored 

let s = Some(String::from("Hello!"));

if let Some(_s) = s {  //
    //this binds _x to the 5 but it gets ignored by compiler ifnot
    println!("found a string");
}

println!("{s:?}");



//ignoring rmaininf parts od a value with ..
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {x}"),
}


//here also .. used to ignore some tuple values
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    //ignore everything in the middle
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

//same can be done lik ehtis
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {second}")
        },
    }
}

//will throw errrp, since .. can only be used once in an expression

/*
EXTRA CONDITIONALS WITH MATCH GUARDS

isa sn addiditonal if condiitition specified after the patternn in a match atm
that must also match for that arm ro be chosen
*/
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("The number {x} is even"),
    Some(x) => println!("The number {x} is odd"),
    None => (),
}



//eample
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

//example 2
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}


//i am also doing  this for my own good