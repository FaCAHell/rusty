fn main(){
    let x = String::from("Star ");
    have_fun(x);
    println!("x: {}", x);
}

fn have_fun(mut y: String){
    y.push_str("{Wars||Trek}");
    println!("y: {}", y);
}
