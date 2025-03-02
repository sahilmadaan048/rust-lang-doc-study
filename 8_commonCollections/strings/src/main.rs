fn main() {
    println!("Hello, world!");

    //creating a string
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    //this works on a literal directly also
    let s = "initial contents".to_string(); //create a string from a strinug literal

    //strings are used so often that there are a bunch of generic APIs availbal eout there to deal with them
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שלום");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");

    //updating a string
    //to concatenate use + operator or format!  macro
    let mut s = String::from("foo");
    println!("{s}");
    s.push_str("bar");
    println!("{s}");

    //push_str takes a string slice (&str) cusu we dont need to take the ownership of the parameter so that
    //we can use that variable later on in our code

    //to push a character
    s.push('l');
    println!("{s}");

    //concatenation woth the + operator or the format ! macro
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    // let s3 = s1 + &s3; //here s1 has been moved here, so it cannot be used further

    //the + operator uses the add method thats called whent  he + operator is used
    //fn add(self, s: &str) -> String {//something in code}
    //this is the signature after we use the + operator to concatenate the stringd

    //since s2 was originally a String , thre &s2 is of type &String, then ehy it does not dive error since
    //the parameter in add method is &str nor #AString
    //
    //the reason is deref coercion which basically means rist automatically converts
    //the refernces of one type into ereferences of another type by claling rhe Deref trait implementation
    //since the first paramter passed is self and not &self, s1 ownership is lost

    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");

    let ans = ss1 + "-" + &ss2 + "-" + &ss3;

    //using the format! macro ffor printinf

    let sss1 = String::from("tic");
    let sss2 = String::from("tac");
    let sss3 = String::from("toe");

    let ans2 = format!("{sss1}-{sss2}-{sss3}");

    /*
    this is just like println! but it retuens a string which can be stpred
    also no ownership ios taken here, thus the varciable being formatted can ve used later in

     */

    //cant access characters from a string using indexing
    let s1 = String::from("hello");
    // let h = s1[0];

    //why indexing does not work
    //a string is a wrapper over a Vec<u8>
    let hello = String::from("Hola"); //its len is 4 wach character takes 1 byte i UTF-8 encoding
    let hello2 = String::from("Здравствуйте"); //its len is 24 and not 12

    //24 is the number of bytes it takes to encode it inUTF-8
    //each unicode scalar value in hello2 string takes 2 bytes of storage
    //
    //
    //thats why the indexes in strings in rust may nor necessarily correlate to a valued

    /*
    this is invalid

    let hello = "Здравствуйте";
    let answer = &hello[0];
        */
    /*
    &"hi"[0] were valid code that returned the byte value, it would return 104, not h.
     */

    //thus to avoid misunderstanding in firther dev process,rust just doesnt compile this at all

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    //it would panic in case we do
    // let temp = &hello[0..1]; //again we are trying to access one byte

    //iterating over strings
    for c in "sahil".chars() {
        println!("{c}");
    }
    // println();
    //
    // //accordin2t to your dmain we could use
    for c in "sahil".bytes() {
        println!("{c}");
    }

    //there is no in it fnctionality to extract the graoheme clusters from the string so leave oit for noe

    //ther are a ton ofuseful functonalituies provided instandard librarylike contains or replace to understand the
    //strings in better
}
// let s3 = s1+&s2; //is more effieicne than coptinh
/*
here s1 ownershi[ is being tken away by add function, then thr opy of the contents of s2 are appended
] and the ownership of the resultant string is returned

 */

/*

strings seem complicated in rust die to the following reasons

1.rusts extensive tendency to give errors
2.strings being complicated
3.UTF-8 encodeing

when discussed about strings, in the further discussion, it could either be a String(int std lib which is
growable, mutable and owned)
or it could be a strinng slice (bowrrowed reference &str)
both Strings and string slices are UTF-8 ecnoded


String is actually implemented internally as a wrsppaer around vector of bytes
withh some extra gurantees, restriction and capabilities



updating a string



 */

/*
to understand why indexing does not work in rist strings

lets understand how strings are implemented in memory



*/

//lets continue our discuusion why rst does not allow indexing

//bytes, scalar values and Grapheme clusters

//computers ultimate stire data as
// bytes [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
// 224, 165, 135]

/*

looking at them unicodesacalar values => rust's char type othey look like

['न', 'म', 'स', '्', 'त', 'े']
                  --       ---  these are not letters(they are diacritics)

graoheme clusters
["न", "म", "स्", "ते"]


A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)).
But it isn’t possible to guarantee that performance with a String,
because Rust would have to walk through the contents from the beginning to the
index to determine how many valid characters there were.


insted be a little mmore speciofic and give a ranmnge
like
let hello = "Здравствуйте";

let s = &hello[0..4];

*/
