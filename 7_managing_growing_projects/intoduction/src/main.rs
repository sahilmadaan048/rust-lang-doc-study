packages crates and modules

group together simmilar things

package = binary crates + 1 library crate

as the packages eveolve together as the project grows,
rust provides us with something called cargo workspaces to manage these

private and public implementation details

concept of scope

when reading, writing, and compiling code, programmers and compilers need
to know whether a particular name is a variable, function, struct, enum, to know
module, constant or other item and what that item means

you cant have two items with the same name in the same scope
tools are available to resolve name conflicts

rust has a well defined module system including

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module