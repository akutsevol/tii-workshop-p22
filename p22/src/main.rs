mod figures;
use crate::figures::{Point, Shape};

fn main() {
    let p = Shape::Point {
        p: Point { x: (1), y: (2) },
    };
    let c = Shape::Circle {
        p: (Point { x: (10), y: (20) }),
        r: (10),
    };
    let t = Shape::Triangle {
        p1: (Point { x: 5, y: 5 }),
        p2: (Point { x: 10, y: 10 }),
        p3: (Point { x: 10, y: 5 }),
    };
    let rc = Shape::Rectangle {
        p1: (Point { x: 15, y: 15 }),
        p2: (Point { x: 25, y: 25 }),
    };
    println!("{}: {}", p.name(), p.gen_string());
    println!("{}: {}", c.name(), c.gen_string());
    println!("{}: {}", t.name(), t.gen_string());
    println!("{}: {}", rc.name(), rc.gen_string());
}
