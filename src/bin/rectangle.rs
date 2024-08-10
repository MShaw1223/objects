fn main() {
    let r = Rectangle {
        height: 3,
        width: 8,
    };
    println!("{}", r.area());
    println!("{}", r.perimeter());
}

struct Rectangle {
    height: i32,
    width: i32,
}
impl Rectangle {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }
    fn perimeter(&self) -> i32 {
        return (2 * self.width) + (2 * self.height);
    }
}
