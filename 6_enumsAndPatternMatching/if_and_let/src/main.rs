#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn main() {
    println!("hello world 2");

    let config_max1 = Some(8);

    //writing using match control flow connsruct
    match config_max1 {
        Some(max) => println!("The maximunm is configured to be {max}"),
        _ => (), //annoying to add even if we are not using this at all
    }

    //wiring using i let
    // let config_max2 = Some(10);
    let config_max2: Option<i32> = None;

    /*

    if let takes in a pattern (Some(max)) and an expresiionn (config_max2)

    if let ensures conciseness, less indentation and less boilerplate code

    but we can no longer do exhautive checking at the compile time only

    if let basically runs the code when the value mataches one pattern and then ignore all other cases

    if let = syntax sugar of match



     */

    if let Some(max) = config_max2 {
        println!("the maximum is configured to be {max}");
    }
    //not writing rhe case for None variant would have given error in match
    //control flow construct since there is exhasutuve checking happening at the compile time
    //compiler checks if all the possible cases are being handled properly at the compile time only
    else if let None = config_max2 {
        println!("the-------------");
    } //this runs just fine

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    println!("count: {count:?}");

    match &coin {
        Coin::Quarter(state) => println!("state Quarter form {state:?}"),
        _ => count += 1,
    }
    println!("count: {count:?}");

    count = 0;
    println!("count: {count:?}");

    //this gives error since value of state was moved earlier during oattern matching
    //way to fix it is use &coin with match or with expression(&coin) in if let

    if let Coin::Quarter(state) = &coin {
        println!("state Quarter from: {state:?}");
    } else {
        count += 1;
    }
    println!("count: {count:?}");

    let tempState = UsState::Alabama;

    //&self will be passed as the first parameter by itself and the second parammeter is year we have defined below
    let year: u16 = 2025;
    let ans = tempState.existed_in(year);

    println!("ans is {ans:?}");

    let description_ans = describe_state_quarter(coin);

    let ans = describe_state_quarter2(tempState);
    println!("{ans:?}");
}

/*
if let syntax lets you handle pattern matching in a less
complicated way while ignnoring the rest




*/

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        println!("abba jabba dabba");
        None
    }
}

fn describe_state_quarter2(state: UsState) -> Option<String> {
    // if let UsState = state {
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."));
        None
    }
    // } else {
    // None
    // }
}

/*
   see the letElse image to learn about that
   and also the summary is there

*/
