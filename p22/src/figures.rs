pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub enum Shape {
    Point { p: Point },
    Circle { p: Point, r: u32 },
    Triangle { p1: Point, p2: Point, p3: Point },
    Rectangle { p1: Point, p2: Point },
}

impl Shape {
    fn area(&self) -> u64 {
        match *self {
            Shape::Circle { r, .. } => (std::f64::consts::PI * (r as f64).powi(2)) as u64,
            Shape::Triangle {
                ref p1,
                ref p2,
                ref p3,
            } => {
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
            _ => 0,
        }
    }

    fn perimeter(&self) -> u64 {
        match *self {
            Shape::Circle { r, .. } => (2.0 * std::f64::consts::PI * r as f64) as u64,
            Shape::Triangle {
                ref p1,
                ref p2,
                ref p3,
            } => {
                let a = p1.distance_to(p2);
                let b = p2.distance_to(p3);
                let c = p3.distance_to(p1);
                a + b + c
            }
            Shape::Rectangle { ref p1, ref p2 } => {
                let width = (p1.x as i64 - p2.x as i64).abs();
                let height = (p1.x as i64 - p2.x as i64).abs();
                (2 * (width + height)) as u64
            }
            _ => 0,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Shape::Point { .. } => "Point".to_string(),
            Shape::Circle { .. } => "Circle".to_string(),
            Shape::Triangle { .. } => "Triangle".to_string(),
            Shape::Rectangle { .. } => "Rectangle".to_string(),
        }
    }

    pub fn gen_string(&self) -> String {
        match self {
            Shape::Circle { p, r } => {
                format!(
                    "x: {} y: {} radius: {} area: {} perimeter: {}",
                    p.x,
                    p.y,
                    r,
                    self.area(),
                    self.perimeter()
                )
            }
            Shape::Triangle { p1, p2, p3 } => {
                format!(
                    "x1: {} y1: {} x2: {} y2: {} x3: {} y3: {} area: {} perimeter: {}",
                    p1.x,
                    p1.y,
                    p2.x,
                    p2.y,
                    p3.x,
                    p3.y,
                    self.area(),
                    self.perimeter()
                )
            }
            Shape::Rectangle { p1, p2 } => {
                format!(
                    "x1: {} y1: {} x2: {} y2: {}  area: {} perimeter: {}",
                    p1.x,
                    p1.y,
                    p2.x,
                    p2.y,
                    self.area(),
                    self.perimeter()
                )
            }
            Shape::Point { p } => {
                format!("x: {} y: {}", p.x, p.y)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figures_names() {
        assert_eq!(
            (Shape::Point {
                p: Point { x: (1), y: (2) }
            })
            .name()
            .as_bytes(),
            b"Point"
        );
        assert_eq!(
            (Shape::Circle {
                p: (Point { x: (10), y: (20) }),
                r: (10)
            })
            .name()
            .as_bytes(),
            b"Circle"
        );
        assert_eq!(
            (Shape::Triangle {
                p1: (Point { x: 5, y: 5 }),
                p2: (Point { x: 10, y: 10 }),
                p3: (Point { x: 10, y: 5 })
            })
            .name()
            .as_bytes(),
            b"Triangle"
        );
        assert_eq!(
            (Shape::Rectangle {
                p1: (Point { x: 15, y: 15 }),
                p2: (Point { x: 25, y: 25 })
            })
            .name()
            .as_bytes(),
            b"Rectangle"
        );
    }

    #[test]
    fn test_figures_areas() {
        assert_eq!(
            (Shape::Point {
                p: Point { x: (1), y: (2) }
            })
            .area(),
            0
        );
        assert_eq!(
            (Shape::Circle {
                p: (Point { x: (10), y: (20) }),
                r: (10)
            })
            .area(),
            314
        );
        assert_eq!(
            (Shape::Triangle {
                p1: (Point { x: 5, y: 5 }),
                p2: (Point { x: 10, y: 10 }),
                p3: (Point { x: 10, y: 5 })
            })
            .area(),
            8
        );
        assert_eq!(
            (Shape::Rectangle {
                p1: (Point { x: 15, y: 15 }),
                p2: (Point { x: 25, y: 25 })
            })
            .area(),
            100
        );
    }

    #[test]
    fn test_figures_perimeters() {
        assert_eq!(
            (Shape::Point {
                p: Point { x: (1), y: (2) }
            })
            .perimeter(),
            0
        );
        assert_eq!(
            (Shape::Circle {
                p: (Point { x: (10), y: (20) }),
                r: (10)
            })
            .perimeter(),
            62
        );
        assert_eq!(
            (Shape::Triangle {
                p1: (Point { x: 5, y: 5 }),
                p2: (Point { x: 10, y: 10 }),
                p3: (Point { x: 10, y: 5 })
            })
            .perimeter(),
            17
        );
        assert_eq!(
            (Shape::Rectangle {
                p1: (Point { x: 15, y: 15 }),
                p2: (Point { x: 25, y: 25 })
            })
            .perimeter(),
            40
        );
    }

    #[test]
    fn test_figures_point() {
        let p = Shape::Point {
            p: Point { x: (1), y: (2) },
        };
        assert_eq!(p.gen_string().as_bytes(), b"x: 1 y: 2");
    }

    #[test]
    fn test_figures_circle() {
        let c = Shape::Circle {
            p: (Point { x: (10), y: (20) }),
            r: (10),
        };
        assert_eq!(
            c.gen_string().as_bytes(),
            b"x: 10 y: 20 radius: 10 area: 314 perimeter: 62"
        );
    }

    #[test]
    fn test_figures_triangle() {
        let t = Shape::Triangle {
            p1: (Point { x: 5, y: 5 }),
            p2: (Point { x: 10, y: 10 }),
            p3: (Point { x: 10, y: 5 }),
        };
        assert_eq!(
            t.gen_string().as_bytes(),
            b"x1: 5 y1: 5 x2: 10 y2: 10 x3: 10 y3: 5 area: 8 perimeter: 17"
        );
    }

    #[test]
    fn test_figures_rectangle() {
        let rc = Shape::Rectangle {
            p1: (Point { x: 15, y: 15 }),
            p2: (Point { x: 25, y: 25 }),
        };
        assert_eq!(
            rc.gen_string().as_bytes(),
            b"x1: 15 y1: 15 x2: 25 y2: 25  area: 100 perimeter: 40"
        );
    }
}
