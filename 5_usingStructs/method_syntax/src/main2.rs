/*

Methods with more paramters



When we run this code with the main function
in Listing 5-14, we’ll get our desired output.
 Methods can take multiple parameters that we add
 to the signature after the self parameter, and those
  parameters work just like parameters in functions.
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        //immutabke borrow of the a=other rectangle also as a paramter

        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
