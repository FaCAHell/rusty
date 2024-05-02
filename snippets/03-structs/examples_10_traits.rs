trait Area {
    fn area(&self) -> f32 {
        0.0
    }
}
struct Rectangle {
    width: f32,
    height: f32,
}
impl Area for Rectangle {
    fn area(&self) -> f32 {
        &self.width * &self.height
    }
}
fn print_area<T: Area>(object: &T) {
    // inlined bound
    println!("The area of this object = {}", object.area());
}
fn main() {
    let rect1 = Rectangle { width: 10.1, height: 3.75 };
    print_area(&rect1);
}
