use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    common::setup(); //will run
    assert_eq!(result, 4);
}

//cargo turns each of these files in the test directory into a crate

/*
we have to bring our adder library into scope at the top of this file
beacuse every file in this test folder is gonna be a new crate


here we dont have to write a tests module with the config annotation v=beacuse
know that all the files in the test directoru are tests
*/


/*
to share the common code between miltiple files, it is a common norm to make a file named commmon.rs 
but if do so cargo wull treat the common.rs file as a integration test file to be testd which we dont want

this we make a folder named commmin inside the test fiolder
ans create a fike named mod.rs inside it

now things will work fine because the sub directories inside the tst foolder are not teated as crates
thus they dont get tested on cargo test


*/


/*
one thing to note here is that we have a lib.rs which means it is a library crate and 
not a binary crate

main.rs would have meant a binary crate a and we cant directly test
biary crates with integration tests

this is why its common to see a vinary crate which is a thin wrapper around a
library crate so that you can test the library crate with the integration tests

*/