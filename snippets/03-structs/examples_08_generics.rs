struct Rectangle<T> {
    width: T,
    height: T,
}
impl<T> Rectangle<T> {
    fn get_width(&self) -> &T {
        &self.width
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 10.1,
        height: 3.75,
    };
    println!("width = {}", rect1.get_width());
}
