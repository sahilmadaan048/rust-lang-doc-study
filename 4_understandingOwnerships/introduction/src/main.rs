fn main() {
    /*
    ownership basically defines how rust program manafes memory

    models are garbage collector(java), manualdeletion of memory (c++, c)
    int rust has ownsership concept set if rules checked by compiler

     */

    //each value in RUST has an owner
    //there can only be one owner at a time
    //when theowner goes out of scope the value will be dropped

    {
        // s not valid here since s not declared here yet
        let s = "hello"; //s is valid from this pnt forward
    } //this scope is now over, and s in no longer valid

    //The string type
    //string literala (hardcoded values) are immutable
    //this we have a differenev data type called String
    //string from a string literal
    // let s = String::from("Hello world");
    // :: allows us to namespace this particaulr from funciton under the string type
    let mut s = String::from("hello");
    s.push_str(", world!"); //appends  a strig literal to a string
    println!("{s}");

    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no

    //drop function is called by RUST as sson as the curly bracket closed
    //its RAII IN c++ (resource acquisition is initialisation)

    let num1 = 5; //simple datac= type
    let num2 = num1; //memory allocated on stack

    let s1 = String::from("hello");
    // let s2 = s1; //prone to oduble free error thus RUSST does not consider s1 valid aymore
    let s2 = s1.clone(); //prone to oduble free error thus RUSST does not consider s1 valid aymore
    println!("{s2}");
    println!("{s1}"); //
    // longer valid
    // let s1 = String::from("hello");
    // let s2 = s1.clone(); //heap data gets copied deep copy
    // println!("s1 = {s1}, s2 = {s2}");

    //string can be mutated but not string literals

    //reason lies in how these two deal with the memory

    //stringliteralswe know the contents at compile timethis iswhy the text is hardcoded into the executable file, thus they are efficient and fast
    //these come from string literals immutability
    //String => mutable, growable piece of text => on heap
    //this memory is requested from the memory allocator at runntime
    //we need to return this memory to the memory allocatpor when we are done with our String

    /*
        freeing up the space should not be done too early (invalid variable case),
        if we forget, we will waste memory
        doing it twice is a bug too
        *
        *must be only 1 free for 1 allocation

    in rust that memory is freed as sonn as the variable that owns it gies out of scope

    /*

        makes RUST memory safe without usinggarbage collector
        we will learn about ownership, borrowing, slices and how rust lays dara out in memory
    */

       Keeping track of what parts of code are using what data on the heap, minimizing the amount of
       duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of
       space are all problems that ownership addresses.

    */

    //if a type implements copy trait, then the vatriable of that type do not move
    //this making them still valid after the move
    //
    //
    //we cantannotate Copy trait to a type if a part of the type hasimplemented the drop
    //trait
    //
    //more about this is available onAppendixC
    //
    //https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html
    //
}
