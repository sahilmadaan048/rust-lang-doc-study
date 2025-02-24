fn main() {
    let mut x = 10;

    println!("the value of x is: {x}");
    x = 5;
    println!("the value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    //shadowing
    let a = 5;
    let a = a + 1; //the second variable with the same name overshadows the variable with the same name writtern first beofore it
    {
        let a = a * 2;
        println!("the value of a in the inner scope is: {a}");
    }
    println!("the value of a is: {a}");
    
    //shadowinttf is different from using mut eyword cus we will get compile time error if uwe try
    //to update the varibale a=a+1 in the next line without using the let keyword
    /*
    
    The other difference between mut and shadowing is that because we’re effectively 
    creating a new variable when we use the let keyword again,
     we can change the type of the value but reuse the same name
    
     */
    
    let spaces = "  ";
    let spaces = spaces.len(); //this is what is possible using shadowing change of data types
    
    
    /*
    let mut spaces = " ";
    spaces = spaces.len();   
    
    
    //will give error
    
     */
}

/* variables in RUST are immutable by defautl
      that means as soon as a variable is associated with a varianle
      iit is immutable
*/

/*
having immutable variables by default helps
by keeping trac of the changes happepning in the va;ues of the variables this
we dont have to eep a trac of it ourselves


 */

/*

constants lie immmutable variables these are also values associated with a names but
are not allowed to changes

but there are a few difference s beteween them


1=>cant use mut with constants
2=>constanrs are always immutaables unlie variables which are immutable by defualr
=>const e=eyword is used for them
=>The last difference is that constants may be set only
 to a constant expression, not the result of a value
 that could only be computed at runtime.



 Rust’s naming convention for constants is to
 use all uppercase with underscores between words.
 */

/*
Constant evaluation is the process of computing the result of expressions during compilation.
 Only a subset of all expressions can be evaluated at compile-time.

 */
