fn main() {
    /* Won't work cos string does not implement trait - Draw
     let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };
    */


    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}


pub trait Draw {
    fn draw(&self);
}

// Trait objects
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        /*      for component in self.components.iter() {
            component.draw();
        } */

        self.components.iter().for_each(|component| {
            component.draw();
        })
    }
}

// Generic Approach
pub struct Screen2<T> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        self.components.iter().for_each(|c| {
            c.draw();
        })
    }
}

// implementing the trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}
