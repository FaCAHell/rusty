fn main() {
    struct Student(String, String, u64);
    let thomas = Student("Thomas".to_string(),"t@gmail.com".to_string(), 11111111);
    println!("number = {}", thomas.2);
}