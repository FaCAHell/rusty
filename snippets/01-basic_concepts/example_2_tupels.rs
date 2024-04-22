#[allow(unused_variables)]
fn main() {
    // tuple
    let t: (char, bool) = ('♥', true);
    let (x, y): (char, bool) = ('♥', true);
    let tuple_of_tuples = (('♥', true), (500, -1), 2024);
    // destructuring ...
    let (u, v) = t;
    let _ = t.0 == x;
    let _ = t.1 == y;
    println!("tuple_of_tuples.1.0 = {}", tuple_of_tuples.1.0);
}
