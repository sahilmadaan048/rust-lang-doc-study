fn main() {
    println!("Hello, world!");
    // panic!("crash and burn!");
    //will cause the thread main to panic at line 2 after printing the alert message int panic block


    let v = vec![1,2,4];
    v[90];
    //this will cause the panic! call to haopen from the library beause of a bug in our code 
    //not from the line of our code where we explicitly did the panic! call
}
/*
2 ways to panic => do something that causes our code to panic in practice
like doing temp[5] when temp.len() == 2

or explaicitly using the panic! macro
  
these panics, print a failure message, unwind, clean up the stack and quit

env varible to print stack => RUST_BACKTRACE=1 cargo run

*/
/*
Unwinding the stack or aborting in response to a panic
see the unwind image to revise this


*/