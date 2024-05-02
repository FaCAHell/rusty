struct Rectangle<T> {
    width: T,
    height: T,
}
fn main() {
    let mut rect1 = Rectangle {
        width: 4.0,
        height: 10.0,
    };
    let mut square1 = Rectangle {
        width: 4,
        height: 4,
    };
    let mut rect2 = Rectangle {
        width: 'a', // fine !
        height: 'b', // fine !
    };
}