struct User{
    active: bool,
    username: String,
    email:String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User{
        active: true,
        username: String::from("badrose"),
        email: String::from("tifelabs@gmail.com");
        sign_in_count: 1,
    };
    //user is mut
    user1.email = String::from("cloud.sh@yahoo.com");    

    // Creating Instances with Struct Update Syntax
    let user 2 = User{
        active: user1.active,
        username: user1.username;
        email: String::from("lucia.me"),
        sign_in_count:user1.sign_in_count,
    }
}

fn build_user(email: String, username: String) -> User {
    User{
        active:true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Init Shorthand
fn build_user(email: String, username:String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}



