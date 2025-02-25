/*

class is just like a function => starts with fn keyword, has a name , can also have paramters and returns a value

but
unlike normal functions, methiods are written in context of a struct or an enum or a trait object (will learn later)

the only difference is that the first f=paramter is always &self which is kinda like
this pointer in c++ or java
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//lets implement a method on struct rectanglke
//initialising the implementation block for rectangle
impl Rectangle {
    //&self means self : &Self
    //kinda like rectangle: &Rectangle
    //
    //we are using &self cus we dont want to take away the ownership
    //
    //inside the impl black Self is an alias for the type the impl block is for
    //
    //if we eanted to borrow it mutabley we woulf have used
    //&mut self

    fn area(&self) -> u32 {
        self.width * self.height
    }

    /*
    everythiong here will be associated with rthe Rectangle type

    . Methods can take ownership of self, borrow self immutably,
    as we’ve done here, or borrow self mutably, just as they can any other parameter.

    */

    /*
    Having a method that takes ownership of the instance by using
    just self as the first parameter is rare; this technique is usually
    used when the method transforms self into something else and you
     want to prevent the caller from using the original instance after
      the transformation.
     */
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "the area of the rectangle is {} square pixels.",
        rect.area() //The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.
    );
}

/*
The main reason for using methods instead of functions,
 in addition to providing method syntax and not having to
  repeat the type of self in every method’s signature, is for
  organization. We’ve put all the things we can do with an i
  nstance of a type in one impl block rather than making future
   users of our code search for capabilities of Rectangle in
    various places in the library we provide.
s
 */

// ------------------

//a method can have the same name as a struct value like

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

// ----this syntax wont give error

/*
when functions and the field are given same names
most of the times it's to return some some value in the field only

these type of methids/ functions are called getters
these helkp us keep the field values proivate but enable read-only access to the end user
through these mnethods

 */

/*

wjeres thr -> operator

see the arrayOperator image in the main folder inside src
 */
