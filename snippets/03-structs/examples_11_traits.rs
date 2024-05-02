trait Speak {
    fn speak(&self);
}
struct Cat;
impl Speak for Cat {
    fn speak(&self) {
        println!("MEOW!");
    }
}
struct Human {
    name: String,
}
impl Speak for Human {
    fn speak(&self) {
        println!("Hi, I'm {}", self.name);
    }
}
fn main() {
    let garfield = Cat;
    let peter = Human { name: "peter".into() };
    garfield.speak();
    peter.speak();
}
