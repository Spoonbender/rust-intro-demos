use rand::Rng;

struct User {
    username: String,
    age: u32
}

fn main() {
    demo();
}

fn demo() {
    let x = User {
        username: format!("ANONymous_{}", rand::thread_rng().gen::<u16>()),
        age: 42
    };

    let y = fix_username(x);

    print_user(y);
}

fn fix_username(original_user: User) -> User {
    let sanitized_user = User {
        username: original_user.username.trim().to_lowercase(),
        age: original_user.age
    };

    return sanitized_user;
    // original_user is freed
}

fn print_user(user: User) {
    println!("User: {}, Age: {}", user.username, user.age);
}