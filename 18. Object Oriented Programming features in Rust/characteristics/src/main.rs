/*
OOPS Languages

has many definnitions

some may classify RUST as object oriented, some may not
RUT is influenced yb many programming paradigms, including functional (chapter 13)

and object oriented also (will learn about this here)

There are 3 important features in any object oriented programming language
-> OBjects
    made out of data and methods and functions operate on that data
    struct and enums, impl block
    have the same functionality
-> Encapsulation    
    means that the implementataion details of an object are hidden from the
    code using that object
    instead of changing the internals directly, code outside of the object is limited to interacting with the object
    through the public api
    this enables the programmers to change the internals without changing the code which uses that object

    RUST uses pub leyowrd to achieve encapsulation
    
-> Inheritance
    ability of an object to inherit from another object's definitoon gaining the data and behavior 
    of that object without having to define the data and behavior itself

    RUST does not directly have this functionality
    but there are 2 ways to do so depending on why you are searching for ingeritance

        1. code sharing => can be accomplisehed using default trait method implementation
            Limitation => as of now, traits can only deinf methods and not fields, although it can be done will learn in future
        2. polymorphism =. allows you to substitue multiple objects for weach object at tuntime if they share 
            certain characteristics  
            in classical inheritance that characteristics would be parent class

            In RUST this is so different, 
                => we can use generics to abstract away concrete types  and use straight bounds
                to restrict the characteristics of those types 
                In addition to generics, rust also provided trait objects which are similar to genercis
                which use dynamic diapl=atch wheras generics use  static dispatch
*/

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}