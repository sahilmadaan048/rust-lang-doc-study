//The Option neum and its Advantanges Over NUll values

Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_number: Option<i32> = Option::Some(5);

    let some_char = Option::Some('e');

    let absent_number = Option::<T>::None;

    let x: i8 = 5;
    let y = Some(5);
     // For example, this code won’t compile, because it’s trying to add an i8 to an Option<i8>:
    // println!("{}", x + y);
    println!("hello world");
}

//the Option is another enum provuded to us in the standard RUST l
//library

/*
the Option enum type encodes the very common scenario in which a value coulkd be somrthong or
it could be nothing


the first vaue in a empty list is nothing
rust does not have the concept of NULL'

in languages having the NULL property, it is fairly easy to get an error when you try to
use a null value as a non-null value

However, the concept of nulll is still a useful one,
a null is a value that is currently invalid or absent for some reason
 */

/*
the option enum is so popular thaat we dont have to include any linbrary in our
code to bring it into scope
its already included in the scope



 */
