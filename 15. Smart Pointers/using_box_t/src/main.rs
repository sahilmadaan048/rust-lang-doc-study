/*
SMART POINTERS

implemented using strcusts with deref and drop trait
*/

//starting with box smart pointer


fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}//dealloacted box pointer and the heap data also

//5 is on the heap
//while b is a box pointer on stack


//box actual use
enum List {
    Cons(i32, List),  //for simplicity take i32 and not a genericc T
    Nil,
}

use list::{Cons, Nil};
//recursive enums
fn main() {
    let list = Cons(1, Cons(2, Cons(3, Cons(Nil))));  //Nil denotes the end of the list
}

//hiw rust computes the size of non recursive enums
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//which variant will need the most spaace

// /in case of Cinns list enum example Box<list> is a fixed size pointer on heapo which pints to some arbitary amount of data on heao
//we now nknow how much space / memory we need on stack noew

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}