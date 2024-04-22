fn main() {
    //  type inference    
    let x = 2.0; // f64

    // explicit type annotation
    let y: f32 = 3.0; // f32    
    let a: bool = true;
    let b: char = '♥'; // unicode
    let c: i32 = 3;
    let d: f64 = 3.14;

    // type casting
    let e = y as i32;
    println!("e = {}", e);
    let f = e as bool; // fine? 
}
