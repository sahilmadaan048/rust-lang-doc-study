/*
in the state pattern, we have some value which have
internal state and that internal state is represented by
state objects


each state object is responsible for its own behavuor and 
deciding whern to transitipn into another state


the value that holds the state objects knows nothing about the different
behaviors of states or when totransition to differetn statetes 


the benifits of using a state pattern is that when the business requirements change
we dont need to change the code which uses the value


we simply need to change the code inside one of the state objects pr perhaps
add new state objects


blogspot workflow => draft -> review => publish => print 


*/

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}