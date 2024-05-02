struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let mut square1 = Rectangle {
        width: 15,
        height: 15,
    };
    println!("area = {}", square1.area())
}