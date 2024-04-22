fn main() {
    let a = 7;
    if a == 4 {
        println!("a == 4");
    } else if a > 10 {
        println!("a > 4");
    } else {
        println!("a == {}", a);
    }
    if a { // fine ?
        println!("a != 0");  
    }
}
