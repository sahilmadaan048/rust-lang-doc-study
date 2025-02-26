enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    println!("Hello, world!");

    /*
    Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.
     */

    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::v6,   //here IpaddrKind i calledd the identifier of the enum and v6 is caled the variant of this enum
        address: String::from("::1"),
    };
}

/*
    while structs are used to group data together
    enums give a wayof saying a value is one of a set of possihbl

*/


/////////////////////////////////////////////////////////////////////
//insetead of using enum inside a struct to grouop data together,  //
//we can do that using enum omnly and that is mnore concise        //
//                                                                 //                                               //
/////////////////////////////////////////////////////////////////////



    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    
/*
 the name of each enum variant that we define also becomes 
 a function that constructs an instance of the enum. That is
 , IpAddr::V4() is a function call that takes a String argument
  and returns an instance of the IpAddr type. We automatically
   get this constructor function defined as a result of defining the enum.
 */


/*
another advantage of using enum over a staruct : each variant can have different types
and amnonts of associated data like the following
 */

 enum InAddr {
    v4(u8, u8, u8, u8),
    v6(String),
 }
 
 let home = iIpAddr::v4(127, 0, 0, 1);
 let loopback = iIpAddr::v6(String::from("127, 0, 0, 1"));

 //providing different data types like these could not have been possible using structs

/*
the enun to store IP addresses is used so often that there is a 
standard definition in the library we can use


the way it is internaly implementes is using two strusts individually deinfed for the 
two variats of the IP addresses we can store

 */

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


/*
even if we have the internal iomplementation  of ipAddr available to use
by the RUST , we can still create our omn Ipaddr enum according to our needs 
without any cinflict since we did not include the librarries defininf
the internal IpAddr enum definition into our code to begin with



we eeill learn more about bringing types into scope 
in chapter 7

 */
//lets see some more examples


enum message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
Quit has no data associated with it at all.
Move has named fields, like a struct does.
Write includes a single String.
ChangeColor includes three i32 values.
 */

/*
wriring this message euum once is sama as
making 4 different structs lik =e thes

 */

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


/*
easy to define a functiuon while using the enum definiton od 
variants of variables to avoid easiness, and decrease redunancy in the cpde


 */


///--------------implementing methods on enum like methods on Structs

impl Message {
    fn call(&self) {
    // fn call(message: &Message) { //both are the same things
        
    }
}
let m = Message::Write(String::from("hello")); //instance if enum type Message and Write variant
m.call();  //this entire thing Message::Write(String::from("hello")); will be stored in the body of the self
//in the call method

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(text) => println!("{}", text), // Print the stored string
            _ => println!("Not a Write variant"),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello")); // Create an instance
    m.call(); // This will print "hello"
}

//will most likely print the string present in the write variant instance m of enum Message

///-------------------