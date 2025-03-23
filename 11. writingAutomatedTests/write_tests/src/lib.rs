pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


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

pub fn greeting(name: &str) -> String {
    format!("hello {}", name) //wont fail
                              // format!("hello ") //will fail
}

pub struct Guess {
    value: i32,
}

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("guess value must be between 1 and 100, git it ?")
//         }

//         Guess {value}
//     }
// }

//making our assertion more precise
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

//we add this to specify that the tests shouldnt ve inclused in the compiled result
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 9,
        };
        let smaller = Rectangle {
            width: 5,
            height: 6,
        };

        assert_eq!(larger.can_hold(&smaller), true)
    }
    #[test]
    fn larger_can_hold_smaller2() {
        let larger = Rectangle {
            width: 8,
            height: 9,
        };
        let smaller = Rectangle {
            width: 5,
            height: 6,
        };
        assert_eq!(!larger.can_hold(&smaller), true)
        //allows to compare two values
        //using panic wiill directly give failed for the test
        assert_ne!(!larger.can_hold(&smaller), true)
        //assert_ne! asserts that the two values passwd are not equal
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("sahil");
        assert!(
            result.contains("sahil"), //custom failure message
            "greetinf did not contain name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

/*
one thing to know anout the asswert_eq is that the struct of type pf values being passed into
the assert_neq as arguments must implement the two traits => PartialEq and Debug

since most of the primitive data types already have these traits deinfed over them
while defining our own data starcutures or enums , we need to explicitly add these 2 traits for assert_neq
to work as expected

*/

/*
custom failure messages

        assert!(
            result.contains("sahil"), //custom failure message
            "greetinf did not contain name, value was {}",
            result
        );

*/

/*
asserting that a finction panics

    #[should_panic]

this assets that the test case writteen below it m=shoudk oanic


*/

/*
tests returnong a Result type enum
doing this helps use to ise ? operator in the tests
which can be used to propagate errors in case these is some
code within code whichi may return some error and we want the code to panic or handle
the error accordingly for those errors also

*/


/*
The three sections of output include the unit tests, 
the integration test, and the doc tests. Note that if any test in a 
section fails, the following sections will not be run. For example, 
if a unit test fails, there won’t be any output for integration and doc tests 
because those tests will only be run if all unit tests are passing.
*/