/*
DROP TRAIT

can be implemented on any type and it allows to contrrol
what happens when a vlue goes out of scope
*/


//its almost always used wwhenimplementing a smart pointer

//custom behavior we generally want is to deallocate the memory resiurces once a variable gioes out of scope

//this cleanup happens auutomaticallly

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

//the variables are dropped in the oposite order of their cleanup

//doing direct drop using c.drop()  because if wwe explicitly call drop once 
//rust will also be calling the drop again due to its custom behavior which will lead to 
//double free error

//but still to manually drop things manually call the drop function with c /(variable name) as parameter
//like drop(c)

/*
this drop function is different from the drop method associated witht the drop trait 
of a smart pointer

thus it is already included in the prelude and we dont have to manually 
include it into our code manually

*/

/*

drop trait + rust ownership model => means we dont we have to deal eith
        deallocating memory in rust thus 

*/