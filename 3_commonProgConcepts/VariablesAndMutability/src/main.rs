fn main() {
    let mut x = 10;

    println!("the value of x is: {x}");
    x = 5;
    println!("the value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    //shadowing
    let a = 5;
    let a = a + 1;
    {
        let a = a + 2;
        println!("the value of a in the inner scope is: {a}");
    }
    println!("the value of a is: {a}");
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
