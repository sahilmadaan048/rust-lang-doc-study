/*
REFUTABULITY -> whether a pattern might fail to match

*/

/*

patterns are of 2 types => refutable and irrefutable

IRREFUTABLE => patterns that will match for any possible value pa=ssed are irrefytable
    let x = 5;


REFUTABLE => patterns that fail to match for some possoble value are refutable
    if let Some(value) = a;
*/

/*

function parameters, let stements , dor loops -> can only accept irrefutable patterns, beacuse the orogram cant do anything meanIngful when values dont match

if let, whilee let, let-else statememnt => accept both refutable and irrefutable patterns, 


in general , we donnt need to care abour refutable or irrefutable patterns

however the concept is important 


*/

let Some(x) = some_option_value;
//wont compile the exprssion after Some(x) may not match with the rhs thingy

//which means rhus is refutable pattern

//but let statemtns only accpt irrefutable patterns


//HOW TO FI IT ->just write if let insteado of ket => converting refutable apttaern o irrefutable
if let Some(x) = some_option_value {
    println!("{x}");
}

//however the ulta => using irrefutable where an refutable is genrally used
if let x = 5 {
    println!("{x}");
};
//only a warinnign xomes and on  compile time error

