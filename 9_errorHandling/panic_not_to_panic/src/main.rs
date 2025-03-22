fn main() {
    println!("Hello, world!");
    //exmples, prototyope code, and test => use panic!
    
      
}


/*
when should we call panic! and when we shoould return Result<T, E>

panic!
Result => gives the calling code options


somtimes the compiler cant tell that the failure is impossiblem but humans can

since returning Result gives us the option to make it unrecoverable using Err case, we can aloways
make our recoverable code into unrecoverable code by  u sing panic! inside the Err of Result enum
this using Result as the retirn ttype for function calls is a good thing in general


*/