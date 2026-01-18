struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("username: {}, email: {}, active: {}, sign_in: {}", user1.username, user1.email, user1.active, user1.sign_in_count);

    let user = build_user(String::from("username"), String::from("user@mail.com"));
    println!("build user: {}", user.username);

    let user2 = User {
        username: String::from("user2"),
        ..user1
    };
    println!("username updated: {}", user2.username);

    let color = Color(12, 16, 20);
    println!("Color: ({}, {}, {})", color.0, color.1, color.2);

    let point = Point(3, 4, 5);
    println!("Point: ({}, {}, {})", point.0, point.1, point.2);

    let subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
