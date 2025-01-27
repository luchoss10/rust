struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sing_in_count: 1,
    }
}

fn build_user_v2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sing_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("luchoss10"),
        email: String::from("email@example.com"),
        sing_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let _user2 = build_user(String::from("other@example.com"), String::from("user_2"));

    let _user2_v2 = build_user_v2(String::from("ver2@example.com"), String::from("name_v2"));

    // let user2_2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("new@example.com"),
    //     sing_in_count: user1.sing_in_count,
    // };

    let _user2_3 = User {
        email: String::from("last version user2"),
        ..user1
    };

    // Struct de tuplas
    //

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    println!("Hello, world!");
}
