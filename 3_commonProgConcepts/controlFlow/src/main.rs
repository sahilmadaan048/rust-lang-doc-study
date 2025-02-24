fn main() {
    //doing something based on some condiiton

    let num = 3;
    if num >= 3 {
        //arms
        println!("condition was true");
    } else {
        println!("consition was false");
    } //if else not provided and codnition was false if also does nt get executed

    let num: i32 = 3;

    if num != 0 {
        //expecte  d abool but its an integer
        println!("number was three")
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //using if in a let statement
    let condition = true; //means we can reuse the variable ame also
    // let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { '6' }; //will give rror since the data type in both the arms must be same
    // let number: i32 = if condition { 'a' } else { '6' }; //will give error data type of variabke is ii32 whilein the arms the retuen vaue is a character
    let number: i32 = if condition { 5 } else { 10 }; //will not give error
    println!("{number}");

    //repition will loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // return (counter * 2); //this gives error here cus it is not a function with a return type
        }
    };

    println!("The result is {result}");

    //we can useloop labels to fix which loops do wewant to break from or cnitinue for that matter

    let mut count = 0;
    'counting_up: loop {
        println!("count= {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break; //without any loop label specified will print the inner loop pnly
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count= {count}");

    //condidiontl loops with while
    let mut num = 3;
    //rin only while the cinsition is true
    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    println!("LIFTOFF1");

    //loopign through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    /*
        while loops are error pron, we could cause the program to panicif the
        indexvalueor test condidion is incorrect

        for ex if later on you removed oneelement from th array but
        forgit to change ndnex<4
        in the while loop condidiojn
        the code would panic

    */

    /*
        that i sthe reason why for oops are considered to be more safe
        and less error prone


        we can also define the range in the fo loop using Range

    */
    println!();
    for element in a {
        println!("the value is: {element}");
    }
    println!();
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!();
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    // println!("LIFTOFF!");
}
