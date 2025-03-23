// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


//unit tests
pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

//cfg memans configiration whicbh has a attribute named test which means 
//cargo will run this test only when we use cargo test

#[cfg(test)]
mod tests {
    use super::*;  //using glob * operator to access the parent module from the children module veen private fields

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}

/*
the rust community thinks about the tests in 2 main categories

unit tests 
    small, focussed and test ont modulek module in isolation and coulr test provate interfaces

integration tests
    completely external to our liibrary, and thius test the public interfaces
    og out=r library

until now we have been wrirint unit tests which are written in the same file as our product code


*/

/*
    you could also put your tests in a different file or folder than the prodict cide
    but rust doesn't make this any easy for you 

    thus it is a conventipon to define tests in the same file as product code in rust at least
*/

//integration tests

/*
defined inside the tests fiolder inside the root directory
*/
