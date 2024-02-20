struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

<<<<<<< HEAD
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);



fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let a = build_user();
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}
=======
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("another@email.com");
    println!("{user1}");
}


>>>>>>> 7706a7a (study result and chapter 4)
