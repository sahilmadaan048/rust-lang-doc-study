pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


//cargo test commans

//iltering the tests

/*
cargo test --help
cargo test one_hunndred  => to run one_hundred wala test only
cargo test add  => Filtering to Run Multiple Tests
cargo test tests  => Filtering to Run tests based on modules they are kept in

*/

//resulting test binary
/*
cargo test -- --help
cargo test -- --test-threads=1
cargo test -- --show-output    //to show whaterver written in println! statements also in the code 
cargo test -- --ignored  ==> runs the test with the ignore attribute specified on it
*/

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}


// Running a Subset of Tests by Name
pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}


//ignoring using ignore attricbute for a test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
} 

// Testing Equality with the assert_eq! and assert_ne! Macros

/*

To perform this test more conveniently. These macros compare two 
arguments for equality or inequality, 

Note that in some languages and test frameworks, 
the parameters to equality assertion functions are called 
expected and actual, and the order in which we specify the 
arguments matters. However, in Rust, they’re called 
left and right

*/
pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}



// Adding Custom Failure Messages
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    );
}


// Checking for Panics with should_panic
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}


// Using Result<T, E> in Tests
#[test]
fn it_works() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

//we cant use #[should_opanic] annotation here o tests that use Result enum
// assert!(value.is_err()).


/*
cargo run => comiler your code and tgen runs the resulatatnt binary
cargo test => compiles code in test mode and runs the resultant test binaty

these tests are run in parallel and the corrsponding outputs are caight

which (default behaviour) can be changed

some options go to cargpop test and some to resultant test binaty


*/

// Running Tests in Parallel or Consecutively
$ cargo test -- --test-threads=1
/*
by default the tests run in parallel using threads, meaning they finish running faster and you get feedback quicker

but since they are running in parallel we need to make sure that they dont share something 
with each other at the same time


say one thread is reading from a file in disk ehile some other thread is writing to the
same file ata the same time
which may lead to inconsisitencies

to restruct the number of threads we can have in parallel to run the tests use the above written command

we we dont want any parallism at all set the value of threads=1
*/



// Showing Function Output
$ cargo test -- --show-output

// Running a Subset of Tests by Name
$ cargo test one_hundred

// Filtering to Run Multiple Tests
$ cargo test add

// Ignoring Some Tests Unless Specifically Requested
// use ignore
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}

// run the ignored test case
cargo test -- --ignored: