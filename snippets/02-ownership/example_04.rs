fn main(){
    let mut s = String::from("<3");
    s.push_str(" <3");

    // share some love
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}, {}", s,r1,r2);


    // let r3 = &mut s;
    // println!("{}, {}, {}, {}", s,r1,r3);
}
