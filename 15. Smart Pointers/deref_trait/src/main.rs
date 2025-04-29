/*
smart pointers are structs whichimplement the deref trait and the drop trair
*/

//DEREF TRAIT
/*
this allows us to treat the pointers as regular references


*/

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y);  //will give error since 5 is a claue and y is reference to a value
    //lets make the y as a smart pointer
}

fn main() {
    let x = 5;
    let y = Box::new(x);  //box is pointing to a copy of 5
                            //since when the primitives like an integer get passed to a function, they get copied
                            //instead of ownership being transferred
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y);  //again giving error
}


//lets create our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; //associated type associated to the genric T 
                        //slighlty different way of declaring a generic parameter
    fn deref(&self) -> &Self::Target {
        &self.0 //we onlu habe one time
    }
}


//without the deref yttrait rust only know s how to deref references, 
//if we want to deref the smart pointers also , we need to implemente the deref trait on them
//to get a refence whjich the compiler knows how to derefernce

//on tunnnf the following code =>
assert_eq!(5, *y);

//rust actually calls this code
assert_eq!(5, *(y.deref()));  //deref returns the refernce, bcux of ownershpip thingy, we dont wanna return th e value direcyly whiw ill move out value from inside the box pointer 



/*
deref coercion => is a special feature in those which implement deref trait which imlpicitly
convertes a reference to one type into a reference of another tyoe
*/

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq(5, x);
    assert_eq(5, *(y.deref()));

    let m = MyBox::new(String::from("RUST"));

    hello(&m);  //here it fdoes not give any error ven though &m is a refernce of box type whil hello explects a string sloce

    //MyBox<String> -> &String -> &str  auto matcially derefenced 2 times
    // hello(&(*m)[..]); does the same thing
}

fn hello(name: &str) {
    println!("hello, {}", name);
}