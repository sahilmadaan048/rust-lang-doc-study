we will learn about packages and crates

cargo new (package_name) creates a package

a package can store multiple crates it coould be a binary crate (which we execute)
or a library crate (which is code that could be used by other programmers) 

crates contain modules => allows you to organise a chunk of code and control all the 
provacy rules 

it also has a concept of workspaces for larger projects which allows you to 
store multiple packages inside the workspace 

main.rs is the crate root, that is the commpiler starts at when buillding a crate it also makes 
up the root module of your crate 

similar convention is there for library crates

if there is a source file in src directory(main.rs for binary crate) or 
(lib.rs for the library crate) , rust will automatically createa a binary/ library crates
whith the same name as the source file as the packages

RULES AROUND CRATES  

1. a package must have atleast one crate 
2. a package could have either 0 or 1 library crate 
3. a package could have any number of binary crates 

to create more binary crates, create a folder named bin , here we can define the other files

