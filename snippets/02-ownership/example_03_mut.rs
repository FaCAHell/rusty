fn main() {
    let mut s1 = String::from("Star ");
    println!("{}", s1);
    have_fun(&mut s1);
    println!("{}", s1);
}

fn have_fun(s : &mut String) {
   s.push_str("{Wars|Trek}");
}
