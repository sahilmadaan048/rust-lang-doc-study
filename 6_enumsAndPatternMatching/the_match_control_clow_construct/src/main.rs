#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


/*
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

*/
/*
Rust has an extremely powerful control flow construct called match
 that allows you to compare a value against a series of patterns 
 and then execute code based on which pattern matches. Patterns can
  be made up of literal values, variable names, wildcards, and many 
  other things; Chapter 19 covers all the different kinds of patterns 
  and what they do. The power of match comes from the expressiveness of
   the patterns and the fact that the compiler confirms that all
    possible cases are handled.
*/


/* 
fn value_in_cents(coin: Coin) -> u8 {

    //here the expression after the match keyword can be of any type 
    //unlike in case of condidional control flow where after if the expession must evaluate to a abpoolean value
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
*/
/* 
fn value_in_cents2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1 //means return 1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter => 25,
    }
}
*/
fn value_in_cents3(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}


/*
The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run
*/
// #[derive(Debug)]
// enum Option<T> {
//     Some(T), 
//     None,
// }

fn main() {
    println!("Hello, world!");

    // let val1 = Coin::Penny;
    // let ans1 = value_in_cents(val1);

    // println!("{}", ans1);

    // let val2 = Coin::Penny;
    // let ans2 = value_in_cents2(val2);

    // println!("{}", ans2);


    let val3 = Coin::Penny;
    let ans3 = value_in_cents3(val3);

    println!("{}", ans3);

    
    let val4 = Coin::Quarter(UsState::Alaska);
    let ans4 = value_in_cents3(val4);

    println!("{}", ans4);

    /*MATCHING WITH OPTION<T>

        like matching in enums in RUST, we can also do matchhin gon option
        enum

        instead of coins we will match the variants of Opiton <TT> enum
        here which are Some(T) and None 

        and like pattern mathcing in other enums, matching is exhaustive in 
        Option enum also, thud we have to match all the variants to a valid return type
        of the matching operation

     */

     #[derive(Debug)] // So we can inspect the state later
     enum UsState {
         Alabama,
         Alaska,
         // --snip-- Other states can be added here
     }
     
     #[derive(Debug)] // Derive Debug for easy debugging
     enum Coin {
         Penny,
         Nickel,
         Dime,
         Quarter(UsState),
     }
     
     // Function to return the value of a coin in cents
     fn value_in_cents(coin: Coin) -> u8 {
         match coin {
             Coin::Penny => 1,
             Coin::Nickel => 5,
             Coin::Dime => 10,
             Coin::Quarter(_) => 25, // Using `_` to ignore the UsState
         }
     }
     
     // Function that prints a message for a Penny and returns the value
     fn value_in_cents2(coin: Coin) -> u8 {
         match coin {
             Coin::Penny => {
                 println!("Lucky penny!");
                 1
             }
             Coin::Nickel => 5,
             Coin::Dime => 10,
             Coin::Quarter(_) => 25, // Fixed: Included missing Quarter case
         }
     }
     
     // Function that prints the state for a Quarter and returns the value
     fn value_in_cents3(coin: Coin) -> u8 {
         match coin {
             Coin::Penny => 1,
             Coin::Nickel => 5,
             Coin::Dime => 10,
             Coin::Quarter(state) => {
                 println!("State quarter from {:?}!", state);
                 25
             }
         }
     }
     
     // Function that adds 1 to Some(i) and returns None if the input is None
     fn plus_one(x: Option<i32>) -> Option<i32> {
         match x {
             Some(i) => Some(i + 1),
             None => None
         }
     }
     
     fn main() {
         println!("Hello, world!");
     
         let val1 = Coin::Penny;
         let ans1 = value_in_cents(val1);
         println!("{}", ans1);
     
         let val2 = Coin::Penny;
         let ans2 = value_in_cents2(val2);
         println!("{}", ans2);
     
         let val3 = Coin::Quarter(UsState::Alaska);
         let ans3 = value_in_cents3(val3);
         println!("{}", ans3);
     
         // Matching with Option<T>
         let five = Some(5);
         dbg!(five);
         
         let six = plus_one(five);
         dbg!(six);
     
         let none = plus_one(None);
         dbg!(none);
     }
    
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/*
matches are exhaistive in RUST

*/


/*
    Catch all patterns and the _ placeholder

*/

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),  //here it is passing the vartable to that function
    //catch all arn is must in these cases beacuse matching in RUST Is exhaustive

}



fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}   //here it borrows tha value of the variable itself


// This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed


let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    // ust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: _ is a special pattern that matches any value and does not bind to that value
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}


// _ is called the catch-all pattern




// Finally, we’ll change the rules of the game one more time so that 
// nothing else happens on your turn if you roll anything other than a 3 or a 7.
// We can express that by using the unit value (the empty tuple type we mentioned
// in “The Tuple Type” section) as the code that goes with the _ arm:

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

