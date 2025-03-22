use core::cmp;

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}

//currently there is a lot of duplication happening jst because the data type sof the values in the ist is different


/*
we use generics to create generix definitions of functions, stricts , methods and enums
*/

//to reduce the code we will create a type aparameter (by convention keep it T short and sweet)

//we are using the PartialOrd trait to restrict the ty[es of T to only those whoch are oprdered]
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

//this wont compile becuase comparsions wont happen for type that T may assume
//thus we need to restrict the domain of T to only those data types which are ordered or allow comparisons
//thus we use a trait here called PartialOrd(will study about them mlater)


/*
using in Struct definitions
*/

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

/*
this will throw compile error since x and y should be of the same type
*/

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

/*
this will work in cases even when the types of x and y are of diferent types
*/

//REMARK: if you think you are using too many genertic tyoes inn code , your code may need to be restructured


//ENUM definitions using generics

//option enum
enum Option<T> {
    Some(T),
    None,
}

//result enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

//in method definitions of a struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x  //returns a refernce to the data stired in x 
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}


//explicitly doing this will mean that the other
//points of Point type which take some data type T other than f32
//will not have the function distance_from_origin implemented for them
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
//this works fine



/*
PERFOEMANCE OF CODE USING GENERICS

food new is that using generic types won't make my program run any slower
than irt would woth concrete types


Rust achieves this through monomorphisation of code using generics

Monomorphization is the process of turning generic code 
into specific code by filling in the concrete types that are used when 
compiled. In this process, the  compiler does the opposite of the steps we 
used to create the generic function
*/
 

let integer = Some(5);
let float = Some(5.0);


//means at coompile time following will happend
//compiler will expand the generic definition of Option<T>
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

//see imp/image.png for summary