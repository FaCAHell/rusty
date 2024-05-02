struct Student {
    name: String,
    email: String,
    number: u64,
}
fn main() {
    let mut thomas = Student {
        name: "Thomas".to_string(),
        email: "t@gmx.com".to_string(),
        number: 11111111,
    };
    thomas.email = "t@gmail.com".to_string();

    let thomas2 = Student {
        email: "t2@gmx.com".to_string(),
        number: 2222222,
        ..thomas
    };    

}

fn create_student(name: String, email: String) -> Student {
    Student {
        name,
        email,
        number: 0,
    }
}