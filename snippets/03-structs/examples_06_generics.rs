struct Rectangle<T,S> {
    width: T,
    height: S,
}
fn main() {
    let mut rect2 = Rectangle {
        width: 4.0, // fine !
        height: 4,  // fine !
    };
}
