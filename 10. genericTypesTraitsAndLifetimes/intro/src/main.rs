//takes in a vector of i32 type and returns a i32 integer
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}


//for duplicate handling in RUST, we use generics
//which are abstract stand-ins for concrete types or other poperties

//we will learn about using generics with functions, struct, enum definitions

//then we will learn how to use traits in a generic way
//we can combine generics and traits to put constraints on the type of generics which can be hsed (will se how)

//finally we will discuss lifetimes => tells compiler hown the refernces relate to each other