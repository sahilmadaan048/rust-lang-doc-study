//we have the debug functionality already
//but wr have to do this rto opt in to make that functionality available for our struct
#[derive(Debug)] //deriving the Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {main/main0.png
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    //refactoring the code using tuopkes
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    /*
    In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument. But in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.

     */

    //Refactoring with Structs: Adding More Meaning
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels. ",
        area3(&rect) //borrow an immutable reference oif the struct rather than tajke ownership from it
    ); //this semicolon is also important at the end of every statement

    print_struct();
}

/*
The area function accesses the width and height fields of the Rectangle instance (note that accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs).

 */

/*
adding  useful functionality with derived traits

println! maxro does not work when we try to print out struct as s whole
 */

// ---------This gives error at compile time only

fn print_struct() {
    let rect = Rectangle {
        width: 20,
        height: 40,
    };

    // println!("rect is {}", rect);
    println!("rect is {:?}", rect); //to implement a formatting caalled Debug to the Reactangle struct
}

/*

RUST does not provide default implementation of formatting calked Display
to structs unlike primitive data types where it is more clear how the user wouldk want the output to be printed
but in case of striucts is less clear since there sre a number of possibilties to print
the strict to the end user like using commas


thus structs dont have a provided implementatiopn of Display to use with println!ans the {} [placeholder

]
 */

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
