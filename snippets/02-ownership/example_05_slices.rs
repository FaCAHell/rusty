#[allow(unused_mut)]
fn main() {
    let mut s = String::from("bald geschafft");
    let hello = &s[0..4];
    let world = &s[5..];

    println!("{} {}", hello, world);

    let first = first_word(&s);

    s.clear();
    
    println!("{}", first); 
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
