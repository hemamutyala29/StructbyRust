struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    students.push(Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phone: String::from("123-456-7890"),
        id: 1,
    });

    students.push(Student {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        phone: String::from("987-654-3210"),
        id: 2,
    });

    students.push(Student {
        name: String::from("Charlie"),
        email: String::from("charlie@example.com"),
        phone: String::from("555-555-5555"),
        id: 3,
    });

    students.push(Student {
        name: String::from("David"),
        email: String::from("david@example.com"),
        phone: String::from("111-222-3333"),
        id: 4,
    });

    students.push(Student {
        name: String::from("Eva"),
        email: String::from("eva@example.com"),
        phone: String::from("444-444-4444"),
        id: 5,
    });

    let student_index = 2; // Index of the student to access

    match students.get(student_index) {
        Some(student) => {
            println!("Student Name: {}", student.name);
            println!("Student Email: {}", student.email);
            println!("Student Phone: {}", student.phone);
            println!("Student ID: {}", student.id);
        }
        None => {
            println!("Student not found at index {}", student_index);
        }
    }
}
