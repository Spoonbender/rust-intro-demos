struct Student {
    name: String,
    age: u16,
}

fn main() {
    let x = Student {
        name: "Maayan Hanin".to_owned(),
        age: 35
    };

    print_student(&x);
}

fn print_student(student: &Student) {
    println!("Student Name = {}, Age: {}", student.name, student.age)
}