/*

INTERIOR MUTABILITY

it is a desing patern in rust which lets you ma]utate a the data even tjoigh there arer immutable refernce soto the data
which is typically disallowed by the borrwoeship rules


To mutate data, this pattern uses unsafe code inside a data structure
to bypass the ttpical rule aroung mutatoion and borrowing

unsafe codeis code that is not checked at comoile time for memory safety



even though the bowrrowship rules are not implemented at the compile time,we cann still einforce thwm at the runtime

RefCell smart pointer
    gives sungle ownership ovet the data it holds muah lik ein box

*/


