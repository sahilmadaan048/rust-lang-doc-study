
//lets understand the RUST_BACKTRACE=1 

/*
compiling and running using:
    using cargo
        RUST_BACKTRACE=1 cargo run outside the src directory

compiling using rustc
    rustc main2.rs
    RUST_BACKTRACE=1 ./main2 
to run the file, in case of panic! occuring , we will get the entire stack info
in case the panic! was called from the rust standard library
    */

fn c(num: i32) {
    if num == 22 {
        panic!("number entered");
        // panic!("{} number entered", num);
    }
}

fn b() {
    c(22);
}

fn a() {
    b();
}

fn main() {
    a();
}