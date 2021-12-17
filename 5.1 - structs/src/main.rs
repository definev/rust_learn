fn main() {
    let me = create_user("Bui Dai Duong", "big.plus.uwu@gmail.com");
    let you = create_user_with_no_param_name(
        String::from("Nguyen Van A"),
        String::from("test@gmail.com"),
    );

    let user_clone = User {
        name: me.name,
        ..you
    };

    println!("{} {} {}", user_clone.name, user_clone.email, user_clone.age);
}

fn create_user(name: &str, email: &str) -> User {
    User {
        name: String::from(name),
        email: String::from(email),
        age: 16,
    }
}

fn create_user_with_no_param_name(name: String, email: String) -> User {
    User {
        name,
        email,
        age: 18,
    }
}

struct User {
    name: String,
    email: String,
    age: u16,
}
