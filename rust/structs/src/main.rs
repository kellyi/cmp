struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = create_user(
        String::from("hello@example.com"),
        String::from("hello123")
    );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another123"),
        ..user1
    };

    println!("{}", user1.email);

    println!("{}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(_, x, _) = origin;
    let Color(r, _, _) = black;

    println!("{} {}", x, r);
}
