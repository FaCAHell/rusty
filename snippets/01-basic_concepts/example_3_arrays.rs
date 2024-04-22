fn main() {
    // type signature optional
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let _othernumbers = [6,7,8];
    // index starts at 0
    println!("First element of the array: {}", numbers[0]);
    // all elements init to same value
    let sieve = [true; 5];
    println!("{:?}", sieve);
}
