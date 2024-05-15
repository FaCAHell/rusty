fn main() {
    let x: i8 = 13;
    match x {
        i if i < 0 => println!("negative"),
        2 | 3 | 5 | 7 => println!("prime less than 10"),
        n @ 10..=19 => println!("10 <= {n} <= 19"),
        i8::MAX => println!("max i8"),
        i if (i % 2 == 0) => println!("even"),
        _ => println!("it is just some number"),
        //  ^ allways needed when guards are used
    }

    let x = Some(13);
    match x {
        Some(42) => println!("answer found"),
        Some(_n) => println!("thanks for the fish"),
        _ => (),
    }
}
