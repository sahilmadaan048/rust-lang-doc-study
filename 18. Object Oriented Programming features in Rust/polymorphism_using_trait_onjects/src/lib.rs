/*
RUST does not support classical inheritance but it does suport polymorphism
which means using the same code for multiple data types through TRAIT OBJECTS

*/

pub trait Draw {
    fn draw(&self);
}

// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,  //box mark pointer dyn => dynamic dispatch

//     //every component in components implement Draw trait
// }

impl Screen {
    pub fn run(&self) {
        fr component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        //draw button
    }
}

//why not use generic implementation
//while usingn generics we are limited to one componennt
//we cant make hybrid components (slider + button, etc) stuff like this



// pub struct Screen<T: Draw> {  //traait bound
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

/*
genercis do not have a performance cost but we cant make hybrid components

while vbice versa for trait object implementation
*/