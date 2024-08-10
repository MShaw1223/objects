fn main() {
    println!(
        "{}",
        Container {
            val: "Foo".to_string()
        }
        .summarise()
    );
    let str = Container { val: "Hello" };
    let str_s = str.summarise();
    println!("{}", str_s);
    let i_s = Container { val: 300 }.summarise();
    println!("{}", i_s);
    println!("{}", Container { val: 20.5 }.summarise());
}

// container holds a generic value of type T
struct Container<T> {
    val: T,
}

trait Summary {
    fn summarise(&self) -> String;
}

// generic impl of Summary for a generic type T
impl<T: Summary> Summary for Container<T> {
    fn summarise(&self) -> String {
        self.val.summarise()
    }
}

impl Summary for String {
    fn summarise(&self) -> String {
        format!("Summarising String: {}", self)
    }
}

impl Summary for &str {
    fn summarise(&self) -> String {
        format!("Summarising &Str: {}", self)
    }
}

impl Summary for i32 {
    fn summarise(&self) -> String {
        format!("Summarising i32: {}", self)
    }
}
impl Summary for f32 {
    fn summarise(&self) -> String {
        format!("Summarising f32: {}", self)
    }
}
