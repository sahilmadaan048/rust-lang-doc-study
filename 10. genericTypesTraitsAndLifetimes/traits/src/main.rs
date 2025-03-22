pub trait Summary {
    fn get_author(&self) -> &str;

    fn summarize(&self) -> String {
        format!("{} (Read more)...", self.get_author()) // Fixed incorrect String::from() usage
    }
} // default implementations

trait MyTrait {
    fn demo(&self) -> String {
        format!("my name is sahil madaan")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let content = format!("{}, by {} ({})", self.headline, self.author, self.location);
        content
    }
    fn get_author(&self) -> &str {
        self.author.as_str()
    }
}

impl MyTrait for NewsArticle {
    fn demo(&self) -> String {
        format!("inside mytrait for newsarticle")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// give me a object which implements the summary trait as the input
//traits as parameters
fn news_aggregator(source: &(impl Summary + MyTrait)) {
    println!("there is a new news in the market");
    println!("{}", source.summarize()); // Fixed typo: changed `summarise()` to `summarize()`
}

// write ta summarise function for this
impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content: String = format!("Tweet by {}: {}", self.username, self.content);
        content
    }
    fn get_author(&self) -> &str {
        self.username.as_str()
    }
}

impl MyTrait for Tweet {
    fn demo(&self) -> String {
        format!("inside mytrait for tweet")
    }
}

/*
traits are similar to a feature often called interfaces in some languages, with some differences

trait = some functionality a type has and can be shared by other types

using traits we can restrict a generic type to behave a particular way
*/

fn main() {
    let tweet = Tweet {
        username: String::from("sahil madaan here"),
        content: String::from("i am currently learning traits in rust"),
        reply: false,
        retweet: false,
    };

    let news = NewsArticle {
        headline: String::from("Rust is taking over!"),
        location: String::from("Global"),
        author: String::from("Tech Weekly"),
        content: String::from("Rust is gaining massive popularity in the developer community."),
    };

    // will give a error saying that tweet does not implement a summary
    // news_aggregator(tweet);

    // Now it works since Tweet implements Summary
    news_aggregator(&tweet);
    news_aggregator(&news);

    println!("1 new tweet: {}", tweet.summarize());
}

/*
DEFINE a trait

a type's behaviour consists of the methods we can call on that type
*/

//trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//this will give gande wala error if i am trying to give two reference one of tweey and other of news as input during function calling
//ssince at the end of day news is not equal to type tweet

// pub fn notify<T: Summary>(item1: &T, item2: &T) {
// println!("Breaking news! {}", item1.summarize());
// println!("Breaking news! {}", item2.summarize());
// }

//this thing works fine thoufh
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//     println!("Breaking news! {}", item1.summarize());
//     println!("Breaking news! {}", item2.summarize());
// }

//while using multiple traits on a struct
pub fn notify(item: &(impl Summary + Display)) {}

//or
pub fn notify<T: Summary + Display>(item: &T) {}

//to make ir more readable in case of more trairsdefines use where keuwros
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
//returning rypea that imeplement trits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// However, you can only use impl Trait if you’re returning
// a single type. For example, this code that returns either
// a NewsArticle or a Tweet with the return type specified as
// impl Summary wouldn’t work:
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

// Using Trait Bounds to Conditionally Implement Methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

//see imp/image.png for thw last part
