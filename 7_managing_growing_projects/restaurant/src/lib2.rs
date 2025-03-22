//bringing paths intro scipe with the use keyword

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    //now we dont have to use the entire absolute path to call the add_to_wishlist function
}

//use statement only creates a shortcut for the particular scope in which the use occurs
//thus its beninift can be taken in the above example but not in the next example
//sunce using the customer module has changed the scope in ehichwhich hosting::add_to_wishlist() eas being
//called thus the following code wont commpile

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

//to fix thus either write the
// use crate::front_of_house::hosting;
//for the customer module also
//or use super keyword for the call to reference to the parent scope and thungs would work fnine
// super::hosting::add_to_waitlist();

//for bringing in the functions using use keyword
//its idiomatic to include the parent module

//this can also be done it would give same result its just idiomatic
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}

//but for structs or enums and other items it is mpre idiomatic to include the entire absolute path
//but both would work just fine

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

//ther is no strong reason to do this, its just the convention folks have gottten used to over time
//the only exception to this is when we are trying to being in two items into scope with the same name
//in that case it s better to include the proant scope

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

//or we can use as keyword to chenge the name of one of the item in our local scope
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

//this would work just fine wothout any conflict

//re exporting names ewwith the pub use
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//using pub keywork with use crate::__::...
//to ma ke the name brought into the scope to be public to be able to use in the other crates also
// This technique is called re-exporting because we’re bringing an item into scope but
// also making that item available for others to bring into their scope.

//using external packages
//rand dependency added in the cargo.toml

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
//just add the dependinecy ion the cargop.toml file and using use keyword to being items from their crates into the scope

use std::collections::HashMap;
 
//since the std library is shipped with the rust language itself,wre dont need to explicitly add it in the dependencies
//in the cargo.toml file

//the glob operator *
use std::collections::*;

//to bring all the public items defined in a path into scipe
//this is often used while testing


//separating modules into different files

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//use front_of_house module from the front_of_house module where it is defined as public 