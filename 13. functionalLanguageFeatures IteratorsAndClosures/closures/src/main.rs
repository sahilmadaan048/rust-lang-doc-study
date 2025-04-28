use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
 
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    // let user_pref1 = ShirtColor::Red;
    let giveaway1 = store.giveaway(user_pref1); //it takes in a variant i f type shirtColor enum
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1,
        giveaway1 // "The user with preference {} gets {:?}",  //will ask for display trait to be implemented onn the ShirtColor enum
                  // user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //we dont have to explicityle define the return type like thesehere
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x: u32| x + 1;
}

/*
closures: anonymous functions which capture their environment
kinda like callbacks

the closure caotures an immutabk=le reference to the self Inventory instance
ans passes it with the code we specify to the unwrap_or_else method.

Functions, oin the other hand, are nor able to capoture ttheir environment in this way
*/

/*

CLOSUDE TYPE INFERENCE AND ANNOTATION


differences between functions and cisures

1 -> closures dont usually require you to annotate the types of th e parameters or the returb value like fn functuions do 
2 -> type annotations are required in fn deifinitione becausse the types are part of an explicit interface exposed to users
3 -> closures on other hand aresnt used in an exposed interface like this =: thery are store in vatiables and used without naming rhm and exposifn rhem to user of our library
4 -> closures are typiccally shrt and relevant only witihn a narror context rather than in any arbotaary  scenario
    within theses small contexts => compiler can automatically infer types of the parameters an dthe retuen type ssimilar to how its able to infer the tyoes of most variables
5 -> we can wxplicitly annotate the types if we want but generally dont do this
*/

let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};


/*
fo closre deifinitions, the compiler will infer pne concrete rtpes for each of their parameters and foe their return value
*/
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);

//wll give error for n wice it catches the types of variable to be the poonce which it recognises in the first closire call
//on the furst cloise call  tghe exmapke clisre the coimpiler oinferred the input variable type to be String , when a second call is made ysing a number(orobably u9 or u32)
//compiler wull throw off an errro\

/*
CAPTRURING REFERENCES OR MOVING OWNERSHUP

closures can capture avlues from their environment in 2 ways -> immutable refernce, mutable reference or a owne2rship move

*/
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
//this is the example og c;osure capturong a immutable reference of the environment variable list 

//here inly)borrows giot bind to tbe closudre definiton now we can call this closure using the variable name every tome we want to use ti
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}//this is am example of closure capturing thr miutable reference  if the lir 

/*
here, between the closre defunutuon and  and the closure call, we cand use an nother println statement on the list vroiable since we cant make an inmmutablenreference when there is already an myravle refernce taking place of an variable


use move keytwpord if you want the cloisure to take the ownerships of all the variables it used in the environmnet variables 
this techniques is mostly used when passinf a closure to a new thread to move the data so that ir is owned by tge new thread
*/


use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

/*

MOVING CAPTURED VALUES OUT OF CLOSURES AD THE Fn TRAITS

a closure body can dii anothinf od the following => 
        1. move a captured value out of the scopeclosure, mutate the captured caue s, or netiher move or mutate the vlue
        or capture nothing from the environment to begin with

the way a closure captures and handlescalues fro the environment affects which traits the clisre immplements'
and trairs are how functions, and stricts can specifiy what jinds of closures they can use
    FnOnce => applies to traits that can ve called only once
            this is the closure that moves the captured vaues out of its bpdy and will only implement FnOnce and and none of th other Fn traits beacise it can be called onc  
    Fn -> applies to closres that dont move and they dont mutate thecaptred values as well as closure that captire nothing from the environment => CALLES MORE THAN ONCE 
    FnMut => incudes closures that dont move captured vlues out of their body, but that might mutate the captured values
                => CAN BE CALLED MORE THEN ONCE
*/

impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
} //uses FnOnce for the trait bound

//use case of FnMut instead of FnONce for the trait bound

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}


//FnOnce trait implementtation
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| { 
        //push happeninng on string, teh ownership of non primitive data types will get transferred since Copy trait is not defined gfot them bu default
        //thus next time when the closure al is made, there is no value variable in th main fucntion since its iwnerhip was taken and is now owned by the sort_operations vector
        sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");
}

//using Fn trait in a simlilar exampe
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}

//no move no mutation, nothing kust pure print statements and updation of value of a lcal variavble, that also did not get captured,, wsince
//num_start_operations is a number which is a primitive datat tupes in rust which means it has already copy trait defined on it 
//so in the function call,  its copy is beign passed and not the actual variable (