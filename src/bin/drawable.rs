fn main() {
    let mut shapes: Vec<Box<dyn Drawable>> = Vec::new();
    shapes.push(Box::new(Circle));
    shapes.push(Box::new(Square));
    shapes.push(Box::new(Triangle));

    for shape in shapes {
        shape.draw();
    }
}

struct Circle;
struct Square;
struct Triangle;

trait Drawable {
    fn draw(&self) -> ();
}

impl Drawable for Circle {
    fn draw(&self) -> () {
        println!("Drawing Circle");
    }
}
impl Drawable for Square {
    fn draw(&self) -> () {
        println!("Drawing Square");
    }
}
impl Drawable for Triangle {
    fn draw(&self) -> () {
        println!("Drawing Triangle");
    }
}
