//normal struct

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Using Tuple Structs Without Named Fields to Create Different Types
//
/*
Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
*/

//using small first alphabet of the struct name also workks, so it is just a convention to use it Capital kike in classes inC++1
//
////naming classes in java is a more string process thanks to the internal implementation
///hate java for its unnecessarrily long implementation for even simpler adta structures
///
///😀☹️☹️
///
///
struct student {
    name: String,
    roll: u32,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn tuple_struct() {
    let black = Color(1, 2, 3);
    let origin = Point(3, 4, 5);

    println!("{:?}", black);
    println!("{:?}", origin);

    // black and origin are different types even though they hold the same data
    // let new_point: Point = black;  // ❌ ERROR: expected `Point`, found `Color`

    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);

    // Destructuring
    let Color(r, g, b) = black;
    let Point(x, y, z) = origin;

    println!("Color - R: {}, G: {}, B: {}", r, g, b);
    println!("Point - X: {}, Y: {}, Z: {}", x, y, z);
}

//unit type structs
struct AlwaysEqual;

fn unit_type_struct() {
    let subject = AlwaysEqual; //see the savd image for more theory related to this rom documentation
}

//strcust are kinda like tuples in their functionality to group multiple related values together
//structs are more flecible than tuples
//we dont have to order data in a specifoc manner to access it

/*
ownership of strcut data

see thei image for better undersanding from the documentation


we are niot using references too strings to store adata in structs now
because we jhave not discussed about the cincept of lifetimes (will come later in chapter 10)

basically using kifetimes woth structs ensures that the data referenced by a struct valud for as long as the strict is


 */

struct Student {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

//following will give error since we have not specofod thelifetime
fn struct_ownership() {
    let student1 = Student {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        active: true,
        username: String::from("sahil madaan"),
        email: String::from("madaan.sahil27@gmail.com"),
        sign_in_count: 1,
    }; //semicolon at the end of statements

    //for a mutable instance, we dan do something like thids
    let mut user2 = User {
        active: true,
        username: String::from("sahil madaan"),
        email: String::from("madaan.sahil27@gmail.com"),
        sign_in_count: 1,
    }; //semicolon at the end of statements

    //we can only makek the entir =e instance to be ny=utabkem we cannot makek indiviualemberds of the struct of the struct instance
    //.//to be mutable or immutable
    user2.email = String::from("sahilmadaan.email@gmail.com");

    //ti access a specific value from a struct , we use dot notation
    println!("{}", user1.email);
    println!("{}", user2.email);

    let user3 = build_user(
        "sahi_codes".to_string(),
        "sahilmadaan048@gmail.com".to_string(),
    );
    println!("{:?}", user3.email);
    let user4 = build_user2("sahik_codes", "sahilmadaan048@gmail.com");
    println!("{:?}", user4.email);

    let user5 = build_user3(user1); //will give error if we use user 1 in some next line cus itds inwnership was moved
    // let user5 = build_user3(&user1);
    println!("{:?}", user5.email);
    // println!("{:?}", user1.email)

    let user6 = build_user4(&user2);
    println!("{}", user6.email);
    println!("{}", user6.sign_in_count);

    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);

    let user7 = build_user5(user3.active, user3.sign_in_count);
    println!("{}", user7.email);
    println!("{}", user7.sign_in_count);

    println!("{}", user3.email);
    println!("{}", user3.sign_in_count);

    tuple_struct();
}

fn build_user5(active: bool, sign_in_count: u64) -> User {
    User {
        username: String::from("solo levelling"),
        email: String::from("sololevelling@hianime.com"),
        active,
        sign_in_count,
    }
}

fn build_user4(user: &User) -> User {
    //
    User {
        username: String::from("Monkey D. Luffy"),
        email: String::from("luffyisgoat@gmail.com"),
        active: user.active,
        sign_in_count: user.sign_in_count,
    }
}

/*

a struct is a custom datda type kinda like a class that helps us keep together
and name multiple related vakues that make up a meaningful group


we will learn how ton define and initialise structs, learn funtions/methods associated with them
structs and enum (chapter 6) are very important
*/

fn build_user(email: String, username: String) -> User {
    let user = User {
        active: true,
        username,

        email,
        sign_in_count: 1,
    }; //semmicolon at the end of ststement
    user
}

// ----------------or this
//
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

//Using the Field Init Shorthand
fn build_user2(email: &str, username: &str) -> User {
    let user = User {
        active: true,
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 1,
    }; //semmicolon at the end of ststement
    user
}

// Creating Instances from Other Instances with Struct Update Syntax
// fn build_user3(user1: User) -> User {
//     // --snip--

//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
//     user2
// }
// ----------can do this also

fn build_user3(user1: User) -> User {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 //written at last
                //order of the values in the struct definition does not matter
    };
    user2
}
