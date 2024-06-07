pub struct Point {pub x: u32, pub y: u32}

pub enum Shape {
    Point {p: Point},
    Circle {p: Point, r: u32},
    Triangle {p1: Point, p2: Point, p3: Point},
    Rectangle {p1: Point, p2: Point},
}

impl Shape { 
    fn area(&self) -> u64 {
        match *self {
            Shape::Circle { r, .. } => (std::f64::consts::PI * (r as f64).powi(2)) as u64,
            Shape::Triangle { ref p1, ref p2, ref p3 } => {
                let a = p1.distance_to(p2);
                let b = p2.distance_to(p3);
                let c = p3.distance_to(p1);
                let s = (a + b + c) / 2;
                ((s * (s - a) * (s - b) * (s - c)) as f64).sqrt() as u64
            }
            Shape::Rectangle { ref p1, ref p2 } => {
                let width = (p1.x as i64 - p2.x as i64).abs();
                let height = (p1.y as i64 - p2.y as i64).abs();
                (width * height) as u64
            }
            _ => 0
        }
    }

    fn perimeter(&self) -> u64 {
        match *self {
            Shape::Circle { r, .. } => (2.0 * std::f64::consts::PI * r as f64) as u64,
            Shape::Triangle { ref p1, ref p2, ref p3 } => {
                let a = p1.distance_to(p2);
                let b = p2.distance_to(p3);
                let c = p3.distance_to(p1);
                a + b + c
            },
            Shape::Rectangle { ref p1, ref p2 } => {
                let width = (p1.x as i64 - p2.x as i64).abs();
                let height = (p1.x as i64 - p2.x as i64).abs();
                (2 * (width + height)) as u64
            }
            _ => 0
        }
    }
    pub fn print(&self) {
        match self {
            Shape::Circle { p, r } => {
    
            println!("Circle x: {} y: {} radius: {} area: {} perimeter: {}",
                    p.x, p.y, r, self.area(), self.perimeter());
            }
            Shape::Triangle { p1, p2, p3 } => {
                println!("Triangle x1: {} y1: {} x2: {} y2: {} x3: {} y3: {} area: {} perimeter: {}",
                p1.x, p1.y,
                p2.x, p2.y,
                p3.x, p3.y,
                self.area(), self.perimeter());
            }
            Shape::Rectangle { p1, p2 } => {
                println!("Rectangle x1: {} y1: {} x2: {} y2: {}  area: {} perimeter: {}",
                p1.x, p1.y, p2.x, p2.y, self.area(), self.perimeter());
            }
            Shape::Point {p} => {
                println!("Point x: {} y: {}", p.x, p.y);
            }
        }
    }
}

impl Point {
    fn distance_to(&self, other: &Point) -> u64 {
        let dx = (self.x as f64) - (other.x as f64);
        let dy = (self.y as f64) - (other.y as f64);
        (dx.powi(2) + dy.powi(2)).sqrt() as u64
    }
}
