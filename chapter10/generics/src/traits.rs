use std::{cmp, fmt::Display};

trait Skewable {
    fn skew(&mut self, angle: u8);
}

trait Drawable {
    fn draw(&mut self, motions: &Vec<Point>);
}

struct Point {
    x: usize,
    y: usize,
    color: Color,
}

type Color = (u8, u8, u8);

struct Canvas {
    width: u32,
    height: u32,
    board: Vec<Vec<Color>>,
}

impl Drawable for Canvas {
    fn draw(&mut self, motions: &Vec<Point>) {
        for cell in motions {
            self.board[cell.x][cell.y] = cell.color;
        }
    }
}

impl Skewable for Canvas {
    fn skew(&mut self, by: u8) {
        for row in &mut self.board {
            for col in row {
                col.0 = cmp::max(col.0, col.0 + by);
                println!("{:?}", col);
            }
        }
    }
}

fn draw<T>(val: &mut T)
where
    T: Drawable + Skewable,
{
    val.skew(50);
    val.draw(&vec![Point {
        x: 50,
        y: 50,
        color: (0, 0, 0),
    }]);
}
