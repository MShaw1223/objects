//Traits, polymorphism
use std::f64::consts::PI;

fn main() {
    let r = Rectangle { h: 4.6, w: 2.5 };
    let c = Circle { r: 3.7 };
    println!(
        "
        - Rectangle -
      Area: {}
      Perimeter: {}
        - Circle -
      Area: {}
      Perimeter: {}
        ",
        r.area(),
        r.perimeter(),
        c.area(),
        c.perimeter()
    )
}

struct Rectangle {
    h: f64,
    w: f64,
}
struct Circle {
    r: f64,
}

trait Calcs {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

impl Calcs for Rectangle {
    fn area(&self) -> f64 {
        return self.h * self.w;
    }
    fn perimeter(&self) -> f64 {
        return (2.0 * self.w) + (2.0 * self.h);
    }
}
impl Calcs for Circle {
    fn area(&self) -> f64 {
        return PI * self.r * self.r;
    }
    fn perimeter(&self) -> f64 {
        return 2.0 * PI * self.r;
    }
}
