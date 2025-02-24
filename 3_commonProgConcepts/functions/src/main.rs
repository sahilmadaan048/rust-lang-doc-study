fn main() {
    println!("Hello, world!");
    let y = 6; //is  a statement;

    another_function(); //calling a function is not a statement 

    let x = five(y);
    println!("{x}");
}

fn another_function() {
    //but dfunction definitons aree also statements
    println!("Another function.");
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*

in some languages lke C and Ruby, thessignment of avalue retuen the value


*/

fn main2() {
    //since statemenrs do nor return a value, you cannot assign them to some variable
    // let x = (let y = 6)
    let y = {
        let x = 3;
        // x + 1; //basically means return this value
        x + 1 //basically means return this value
        //expresiions do not includeendingsemicolons

        //placing a semicolon at the end will make ir =t  statementwe will get an error
    };
    println!("the value of y is: {y}");
}

//functions with returen valuies
fn five(temp: i32) -> i32 {
    temp * 5
}
/*

main function is the entru point of the rust progrn
snamke case is followed in rust

functions can have parameter
the concrete values are calledarguments

statements and expressions

RUST is an expression - based language which is a distinction from other languages


statements:are insructions that perform some action and do not return a value
expressions:evelaute to a resultant value

*/
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}

/*

The main error message, mismatched types, reveals the core issue w
ith this code. The definition of the function plus_one says that it will
 return an i32, but statements don’t evaluate to a
  value, which is expressed by (), the unit type. Therefore
  , nothing is returned, which contradicts the function definition and results
   in an error. In this output, Rust provides a message to possibly
    help rectify this issue: it suggests removing the semicolon, which would fix the error.



*/
