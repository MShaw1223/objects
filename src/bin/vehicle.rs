// Traits with default methods, method overriding
fn main() {
    let chopper = Bike;
    let ford = Car;
    chopper.start_up();
    ford.start_up();
}

struct Bike;
struct Car;

trait Vehicle {
    fn start_up(&self) -> () {
        println!("broom broom");
    }
}

impl Vehicle for Car {}
impl Vehicle for Bike {
    fn start_up(&self) -> () {
        println!("whizz")
    }
}
