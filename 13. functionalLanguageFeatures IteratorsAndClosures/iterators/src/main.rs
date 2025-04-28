/*
alows us to perfoerm some task ina s equnce of itemas in a turn\

an iterator is reposndibe for the logic of oterating over each item an fdetermining when the sequcne has finished


in RUST, iterators are lazy which means they have no affect  until you call methoda that consike the iterator to use it up

the following code in itself does not relly do anythinf
*/

let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

//use of iterator
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {val}");
}


/*
THE ITERATOR TRAIT AND THE NEXT METHOD
all iterators implement a trait names iterator
it =s definition looks like this

*/
pub trait Iterator  {
    type Item;  //a tyoe sretuned from the iteratorr
  
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
/*
note here we need to make the iteratpr mutable because on every call of ext the internal state oif the iterator changes every time 

we didnt have to do this in the for loop examkppleof using iterators beacisie
the for loop is taking ownership of the iterator itself

*/

//METHODS THAT CONSUME THE ITERATOR
/*
    methids that call next are called consumming adapters, vbecause calling them uses up the iterator
     one such example is sum mehjod which takes ownership pof theiterator anditerates theorufh the items bu repeatedly calling next

*/


#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    //we cant use vi_iter here beacuse sum method takes away the ownership of the iterator
    assert_eq!(total, 6);
}


//METHODS THAT PRODUCE OTHER ITEARTORS
/*

iterator adopters are methids defined on the iterator trair that dont consume the iterator
instead they produce different iteratoes by changing sime aspect of the original iterator
*/

let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);  //returns a new iterator with the modified vall;ies


//magic happens now
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); //consume this modified iterator by collect method

assert_eq!(v2, vec![2, 3, 4]);


/*
USING CLOSURES THAT CAPTURE THEIR ENVIRONMENT

many iteratior adapters take closures as arguments, and commonly the closures we will speicify as
arguments to iterator adapters wull be closures that capture their environment


filter iterator adpter => takes a closure 
the closure gets an item from the iterator and return sa bool if the closure returns true 
the value will  be included innt he iteration produces by filter
r
*/

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
//shoe struct which has the details what ir has

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}//reutrns  avector  containing the shoes of the specified size

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]; //lets make a vector

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}