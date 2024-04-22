fn main() {
    let s1 = String::from("cats and dogs");
    let (s1, len) = str_len(s1);
    println!("{} has {} chars", s1, len);
}

fn str_len(s : String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
