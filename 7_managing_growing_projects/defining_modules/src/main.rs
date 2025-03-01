// backyard
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── garden
//     │   └── vegetables.rs
//     ├── garden.rs
//     └── main.rs


/*
what we will be learning ?

modules and paths to name items
use keyword to bring path into scope
pub keyword to make items public 
as keyword
external paxkages
glob operator

PROCESS FLOW

1.compilation starts at the crate root
2.declaring modules
3.declaring submodules
4.paths to code in modules
5.private vs public
6.the use keyword


*/

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}