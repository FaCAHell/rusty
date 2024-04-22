fn main() {
    let mut count = 0;
    'outer: loop {
        let mut remaining = 3;
        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count = {count}");
}
