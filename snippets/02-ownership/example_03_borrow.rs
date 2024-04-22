fn main() {
    let s1 = String::from("cats and dogs");
    let len = str_len(&s1);
    println!("{} has {} chars", s1, len);
}

fn str_len(s : &String) -> usize {
    s.len()
}
