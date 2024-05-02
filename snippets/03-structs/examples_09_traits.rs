trait Area {
    fn area(&self) -> f32;
}
struct Rectangle {
    width: f32,
    height: f32,
}
struct Circle {
    radius: f32,
}
impl Area for Rectangle {
    fn area(&self) -> f32 {
        &self.width * &self.height
    }
}
impl Area for Circle {
    fn area(&self) -> f32 {
        3.14 * &self.radius * &self.radius
    }
}
fn main() {
    let rect1 = Rectangle { width: 10.1, height: 3.75 };
    let circ1 = Circle { radius: 5.3 };
    println!("area@rect1 = {}", rect1.area());
    println!("area@circ1 = {}", circ1.area());
}
