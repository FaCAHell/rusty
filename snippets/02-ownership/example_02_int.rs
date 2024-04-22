fn main(){
    let x = 5;
    have_fun(x);
    println!("x: {}", x);
}

fn have_fun(mut y: i32){
    y += 15;
    println!("y: {}", y);
}
