struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

fn main() {
    // Unit-like struct
    let _subject = AlwaysEqual;

    // Tuple structs
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let mut user1 = User {
        active: true,
        username: String::from("badrose"),
        email: String::from("tifelabs@gmail.com"),
        sign_in_count: 1,
    };
    // the whole instance must be mut to change any field
    user1.email = String::from("cloud.sh@yahoo.com");

    // Manually copying fields from user1 (clone avoids moving username)
    let _user2 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("lucia.me"),
        sign_in_count: user1.sign_in_count,
    };

    // Struct update syntax (moves user1.username)
    let _user3 = User {
        email: String::from("jason.me"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Field init shorthand 
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}