use polymorphism_using_trait_onjects::{Screen, Button, Draw};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //draw select box
    }
}

fn main() {
    let screen = Screen {
        components: vec[
            Box::new(String::from("sahil")), //Draw trait has not been implemented for this so yeah cant use this here
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    string::from("yes"),
                    string::from("no"),
                    string::from("maybe"),
                ]
            }),
            Box::new(Button {
                width: 100,
                height:  100,
                label: String::from("ok"),
            })
        ],
    };
    screen.run();
}

/*
this is how the end user can implement their own components

*/


/* ---------------------------- STATIC vs DYNAMIC DIAPATCH ---------------------
MONOMORPHISATION => CH 10

is a process where the compiler will generate nonn- generic implementations of functions
based on the concrete types used in place of the generic types

this is static diapatch => when the compiler knows the concrete functions you are calling at compile time

opposite is dynamic diapatch => compiler does not know the concrete function yoi are calling, at compile time

thus while writing the trait bound implementation, dynamic diapatch is needed becaiuse we will need to write
objects whse discrete types are known only at the urnrime

due to this, there is also a performance cost in the trait object implementayion we saw earlier

but it provides us more flexibility as a developer


*/

/*
Object safety => can only make object safe traits using trait bounds

2 relevant rules

A trait is object safe trait when all the methoda implemented on that trait have these 2 properties

    1. the return type is not self
    2. there are no generic parameters

*/