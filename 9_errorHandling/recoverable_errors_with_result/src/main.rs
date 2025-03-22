// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
/*

like the option enum, the variabts of the Result enum have been included
in the default by default, so we dont have to wriet tjem explivcitly
so we dont have tio use Result::Ok or Result::Err in the arms during
pattern matching


*/
//here T and E are generic types

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };


    let greeting_file_result = File::open("hello.txt");
    

    //matching on differenterrors
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}



// the return  type of File::open() is Result<T, E>
//in case if success the value in that variabe will be an instance of OK that contains a file handle, in the vase
// where it fails the value in the varuablr greeting_file_result will be an instamve of Err
/*
most errors are not serious enough to stop the entire problem
to deal with them

these errors can be easily interpreted and can be easily reponded with
T will be tyoe of value returned on success
E will be the erro type returned in case of failure within the Err variant

*/


/*
we can slo use closures to be an alternative to using match with Result <T, E> enum
thesea are used with any of the  methiods defines on Resukt<T,E>. these methods can be more orecise than 
using match when handling Request<T,E> values in out =ur code

see the image and also main2.rs to see the syntax
*/


//shortciuts for panic on error => unwrap and expect

//using match is a bit verbos abd doesn't always communicate intent
//thus trhe rust standard library peovides with  a number of useful functions 
//with specofic functionalities => unwrap method is a shortcut method implemented
//just like the match expression, if the Result value is the OK craiant, unwrap will
//return the value inside the ok
//if he result is the Err variant unwrap will call the panic! macro for us 

use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
   
    // We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");

}


//propagating errors
//looks so lamba of done everything so sequenctially
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
//we can do error propagating
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
//we can use the ? only where a finction returns a Result enum

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() => Result<String, io::Error> {
    let mut username_file_result = match username_file_result
}                                           


// Where The ? Operator Can Be Used
// The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. This is because the ? operator is defined to perform an early return of a value out of the function, in the same manner as the match expression we defined in Listing 9-6. In Listing 9-6, the match was using a Result value, and the early return arm returned an Err(e) value. The return type of the function has to be a Result so that it’s compatible with this return.

// In Listing 9-10, let’s look at the error we’ll get if we use the ? operator in a main function with a return type that is incompatible with the type of the value we use ? on.

// Filename: src/main.rs
// This code does not compile!

use std::fs::File;
fn main() {
    let greeting_file = File::open("hello.txt")?;
}


//ther is also something called a Box<dyn Error> which we will study later in chapter 18
//it is a trait object and for now can be understood to be any type of error

//after a thorough reading also some things dont make sense
//cuz i have not studied the futher topics like traits and trait objects

//heres the link
https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html