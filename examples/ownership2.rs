struct Student<'a> {
    name: String,
    mentor: Option<&'a Student<'a>>
}

/// Prints student info, and mentor info (if exists)
fn print_student(student: &Student) {
    print!("Student Name = {}",student.name);
    if let Some(mentor) = student.mentor {
        print!(", Mentor: Name = {})", mentor.name);
    }
    println!();
}

fn expell(student: Student) {
    println!("Expelling {}", student.name);
    // student deallocated here as owner's scope ends
}

fn main() {
    let alice = Student {
        name: "Alice".to_owned(),
        mentor: None
    };

    print_student(&alice);

    let bob = Student {
        name: "Bob".to_owned(),
        mentor: Some(&alice)
    };

    print_student(&bob);

    let bob = Student { name: bob.name, mentor: None };

    expell(alice);

    print_student(&bob);
}