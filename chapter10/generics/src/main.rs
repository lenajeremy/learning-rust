mod lifetime;
mod traits;
use std::cmp;

fn main() {
    lifetime::test();
    let my_vec = vec![1, 2, 3, 4, 0, 5, 49, 391];
    let largest_int = largest(&my_vec);

    println!("{:?}", my_vec);
    println!("largest: {}", largest_int);

    let my_chars = vec!['a', 's', 'd', 'f', ';', 'l', 'k', 'j'];
    let largest_char = largest(&my_chars);
    println!("{largest_char}");

    let mut points = vec![
        PointVector { x: 1, y: 5 },
        PointVector { x: 0, y: 0 },
        PointVector { x: 5, y: 20 },
        PointVector { x: -1, y: -5 },
    ];

    println!("{:#?}", points);

    let largest_vector = match largest_index(&points) {
        Some(n) => &mut points[n],
        None => return,
    };

    println!("{:?}", largest_vector);
    largest_vector.rotate(0.5);

    println!("{:?}", largest_vector);
    println!("{:#?}", points);
}

fn largest_index<T: cmp::PartialOrd>(v: &Vec<T>) -> Option<usize> {
    if v.len() == 0 {
        return None;
    }

    let mut max_index = 0;
    for i in 1..v.len() {
        if v[i] > v[max_index] {
            max_index = i;
        }
    }
    Some(max_index)
}

fn largest<T: cmp::PartialOrd>(v: &Vec<T>) -> &T {
    if v.len() == 0 {
        panic!("zero-length vector provided.");
    }

    let mut max = &v[0];

    for num in v {
        if num > max {
            max = num;
        }
    }

    max
}

#[derive(Debug, Clone)]
struct PointVector {
    x: i32,
    y: i32,
}

impl PointVector {
    fn abs(&self) -> f64 {
        let x64 = self.x as f64;
        let y64 = self.y as f64;
        (x64 * x64 + y64 * y64).sqrt()
    }

    //fn half(&mut self) {
    //    self.x /= 2;
    //    self.y /= 2;
    //}
}

impl cmp::PartialOrd for PointVector {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.abs().partial_cmp(&other.abs())
    }
}

impl cmp::PartialEq for PointVector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

trait Rotatable {
    fn rotate(&mut self, angle: f32);
}

impl Rotatable for PointVector {
    fn rotate(&mut self, angle: f32) {
        if angle > 360.0 || angle < 0.0 {
            return;
        }

        self.x *= -1;
        self.y *= -1;
    }
}
