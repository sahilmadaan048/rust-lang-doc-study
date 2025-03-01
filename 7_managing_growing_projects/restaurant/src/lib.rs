mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/*

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment


*/

/*
We define a module with the mod
keyword followed by the name of the
module (in this case, front_of_house).
\The body of the module then goes inside
curly brackets. Inside modules, we can place
other modules, as in this case with the
 modules hosting and serving. Modules
 can also hold definitions for other items, such as
  structs, enums, constants, traits, and—as
   in Listing 7-1—functions.
*/


/*

here hoisting nests insidd front_of_house the tee ahow some modules are siblinfs, meaning
tehy are defines in the same module
the tree also shows that sme modules ares sinlings
meaning htey are defined in the ssme module
hoisting and serving are siblings defined within front_of_house
id mofule A is contained inside module B, we say that module A is thr child of module B 
suchh that module B is the parent of moduke A
Notice that the entire midule tree is rooted under the impplicit module named crate

*/