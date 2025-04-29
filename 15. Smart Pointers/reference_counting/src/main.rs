/*
we will be talking about a special type of smart pointers which allows
us to store the ownership of some data

*/

/*

to enable multiple ownerships of a value we can use a refeenrce counting smart pointer 
which keeps the track of number of references to a value


and when this count becomes zero the value will get cleaned up

*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); //here a has been moved into b
    let c = Cons(4, Rc::clone(&a));  //here c now owns a
}
/*
the clone associated method on teh Rc (Reference counting) smart pojnter
unlike all other clone implementations in rust

this ::clone () method just increase the countof the number of owners of this value
another way to do this is to calll a.clone()



*/

/*

for this reason a now can not be used in line 28 where we try to create list c

now we could change the defintion of Cons function by using reference instead of owned 
but that woukd require the concep tof lifetines'

using lifetimes we can specify that every element in the list has to live atleast as long as the entire list


*/

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}





