fn max<T: PartialOrd>(a: T, b: T) -> T { // inlined
    if a > b { a } else { b }
}
fn main() {
    let a = 5;
    let b = 7;
    println!{"max = {}", max(a, b)};
}