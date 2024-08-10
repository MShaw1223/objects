use std::f64::consts::PI;

fn main() {
    let c = Shape::Circle { radius: 4.8 };
    let s = Shape::Square { length: 7.8 };
    let t = Shape::Triangle {
        base: 4.2,
        height: 6.2,
    };
    println!("Circle Area {}", c.area());
    println!("Circle Perimeter {}", c.perimeter());
    println!("Square Area {}", s.area());
    println!("Square Perimeter {}", s.perimeter());
    println!("Triangle Area {}", t.area());
    println!("Triangle Perimeter {}", t.perimeter());
}
enum Shape {
    Circle { radius: f64 },
    Square { length: f64 },
    Triangle { base: f64, height: f64 },
}

trait ShapeProperties {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

impl ShapeProperties for Shape {
    fn area(&self) -> f64 {
        match self {
            Self::Circle { radius } => PI * radius * radius,
            Self::Square { length } => length * length,
            Self::Triangle { base, height } => base * height / 2.0,
        }
    }
    fn perimeter(&self) -> f64 {
        match self {
            Self::Circle { radius } => PI * 2.0 * radius,
            Self::Square { length } => 4.0 * length,
            Self::Triangle { base, height } => {
                let hypotenuse = (base * base + height * height).sqrt();
                base + height + hypotenuse
            }
        }
    }
}
