enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new(); //basically telling the rust compiler that we are storing i32 type elements
    //we can also use vec! macro
    let v2 = vec![1, 2, 3];

    //updating a vector
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let temp = vec![1, 2, 3, 4, 5, 6];

    //this gives us a reference to the value stored at that index
    let third: &i32 = &v[2]; //it also follows 0 based indexing
    // let third: &i32 = &v[10]; //it also follows 0 based indexing

    /*


    thread 'main' panicked at src\main.rs:17:25:
    index out of bounds: the len is 4 but the index is 10
         */
    println!("the third elements is {third}");

    let third: Option<&i32> = v.get(2);
    //unlike the get mthod which returns an option enum whrn called with the index as the parameter
    //where we can further ise match to do pattern matching
    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("there is no third elemenrt"),
    }

    //iterating over a vector
    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{i}");
    }
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //when the vector gets dropped, all of its contents are also dropeed meaning the integers it olds are also cleaned uo
    //the borrow checker ensures that any references to contents of a vector are used only whn the vector itself is valid
    {
        let v = vec![1, 2, 3, 4];

        //do stuffs with v
    } // <= v gies out of scope and is freed here

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
    //will throw error
    //
    /////
    // //
    /////
    /*

    When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure this reference and any other references to the contents of the vector remain valid
     */

    /*

    when we try to access the 101th elemets in the last section of the code, the first one will get panic
    because it is referncing a nonexistent elements
    this method is good when we want to crash our program in case someone is trying to access some value at an
    index larger than the array size


    in the 2nd case since the index is searched through the= array using the get method it means
    it returns an option enum, which means in case of index>n it will return None
    instead of panicking
    use this code when the index is being accessed a number of times, and thus we must add logic
    to handle having either Some(&element) or None

    For example, the index could be coming from a person entering a number. If they accidentally enter a number that’s too large and the program gets a None value, you could tell the user how many items are in the current vector and give them another chance to enter a valid value. That would be more user-friendly than crashing the program due to a typo!

     */
}

/*

here we are learning about vectors
they allow us to stire  more than one value of a single data type next to each other in memory
internally vectors in rut are implemented using genericcs (kinda the same as in c++)

much like any otyher data tyoe we need to use the mut keyword wih the instances of this type cus
by default teh vraibles are immutable in rust


there are 2 ways to access a refernce a value stored in a vector either by using indexing
or using the get method
 */

/*
we can also create a enum to store different variants or different data typ es of elementds ib the same vectote
it is kinda like a cheat code to make vector store different data types
since enum allows us to store different data types and give them names(edge over structs)
calling them variants

*/

/*

Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. We must also be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled, as discussed in Chapter 6.

If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object, which we’ll cover in Chapter 18.
 */

/*

DROPPING A VECTOT DROPS ITS ELEMENTS


 */
