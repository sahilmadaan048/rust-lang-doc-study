Rust requires usto acknowledge the possiblity of n errpr and 
take some action befoere your code will compile

makes our code more ronust since we handkek these possible errors 
appropriately before deployig our code to the production !  

Types of errors 
1. recoverable
    ->file not found error
    ->just report the end user and retry the operation
     
2. unrecoversable
    ->trying to access a location beyond the size of an array (symptons of a bug)
    ->immediately stop the program in this caze

most languages dont distinguish them like that and handle them both in the same way
using mechanisms such as excepotions

rust does not have exceptions, instead it has rhethe type Result<T,E> for recoverable errors
and the panic! macro for thre unrecoversable errors


