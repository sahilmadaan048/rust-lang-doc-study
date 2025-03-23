/*
we need to write tests to check if our code lofic is working fine
*/


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4)
    }
    #[test]
    fn it_works2() {
        panic!("2+2")
    }
}

// Checking Results with the assert! Macro

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

//this will fail no changing the comparison operator in the funcion can_hold() 
//  --snip--
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}



// Testing Equality with the assert_eq! and assert_ne! Macros