mod blog;
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }
}

struct Rectangle;
struct Square;

impl Draw for Rectangle {
    fn draw(&self) {
        println!("The rectangle is being drawn");
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("The square is being drawn");
    }
}

fn main() {
    let mut screen = Screen { components: vec![] };
    let rect = Box::new(Rectangle {});
    let square = Box::new(Square {});

    screen.add_component(rect);
    screen.add_component(square);

    screen.run();
}
