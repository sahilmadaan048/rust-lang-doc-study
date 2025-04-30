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

    thios is difefrebnt from the Biooc smart pointer sdince that implements 
    bowrrow rules at compile time wghikle this iomplements borrow r=ules at the runtime

wer cAN USE HTIS rEFGcELL SMART PPOINTER ONLY IN SINGLKE THREADED PROGRTAMS FOR NOW AAR LEAS5T


since refcwell does thj checks at the runtinme we can have the
data insider it to be mutabkle while the refcell istelf as a smarft  pointer will be immutabkle




*/

/*
contrast this to the box smart pointer which allows mutabke or immutabkle borrrows at the comouile ntime ]
nut if we want to make the data inside the box smart opinter to be mutable, we will have
top make the bx smart pooinmter itself as mutable

*/


/*

mutating a value inside an immutablkke value is called the interior mutability patternz

*/

// fn main() {
//     let x = 5;
//     let y = &mut x;
// }

//interrior mutability

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct MockMessenger {
//         sent_messages: Vec<String>,
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: vec![],
//             }
//         }
//     }

//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             self.sent_messages.push(String::from(message));
//         }
//     }

//     #[test]
    // fn it_sends_an_over_75_percent_warning_message() {
    //     let mock_messenger = MockMessenger::new();
    //     let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    //     limit_tracker.set_value(80);

    //     assert_eq!(mock_messenger.sent_messages.len(), 1);
    // }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &s tr) {
            self.send(lf.sent_messages.borrow_mut().pushString);
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}