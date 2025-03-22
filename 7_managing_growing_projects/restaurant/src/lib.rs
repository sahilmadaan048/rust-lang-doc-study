mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
//will give error sunce cant access the child modules without using the pub keywoes


mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

//using the super keyword to reference an item that we know is in the parent midule which can meke
//reaaranging the module tree easier when the midules is clsoely related to the parent but
//module might be moved elsewhere in the module tree someday

/*

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment


*/

/*
We define a module with the mod
keyword followed by the name of the
module (in this case, front_of_house).
\The body of the module then goes inside
curly brackets. Inside modules, we can place
other modules, as in this case with the
 modules hosting and serving. Modules
 can also hold definitions for other items, such as
  structs, enums, constants, traits, and—as
   in Listing 7-1—functions.
*/


/*

here hoisting nests insidd front_of_house the tee ahow some modules are siblinfs, meaning
tehy are defines in the same module
the tree also shows that sme modules ares sinlings
meaning htey are defined in the ssme module
hoisting and serving are siblings defined within front_of_house
id mofule A is contained inside module B, we say that module A is thr child of module B 
suchh that module B is the parent of moduke A
Notice that the entire midule tree is rooted under the impplicit module named crate--

*/



//making structs ans enums public

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}



//in the enums
//enums wouldnt be any useful if there variabmts are private 
//thus as somm as we use the pub keyword for the struct
//we can access it svariabnts also wihthout explicitly mentioning the pub keyword for themunlike structs where we have to do so

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//use keyword