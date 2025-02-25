/*
Associated funtions

All functions defined within an impl block are called associated functions because
they’re associated with the type named after the impl


we can define associated functions that dont have self as their first
parameter (ans thus are not methods becuase they dnit need an instance of the type top
work with)

for rest see the associatedFunctions image in main3 folder in the src folder
 */

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/*

Structs let you create custom types
that are meaningful for your domain.
By using structs, you can keep associated
 pieces of data connected to each other
 and name each piece to make your code clear.
 In impl blocks, you can define functions that
 are associated with your type, and methods are
 a kind of associated function that let you specify
 the behavior that instances of your structs have.

But structs aren’t the only way you can
create custom types: let’s turn to Rust’s enum
 feature to add another tool to your toolbox.
 */

//------making multiple impl blocks for one struct is a valid syntax in RUST

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
