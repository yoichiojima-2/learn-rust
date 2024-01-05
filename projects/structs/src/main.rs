fn main() {
    use_struct();
    use_struct_2();
    use_tuple_struct();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn use_struct() {
    let mut user1 = User {
        email: String::from("yoichiojima@gmail.com"),
        username: String::from("yoichiojima"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("username = {}", user1.username);
    println!("email = {}", user1.email);
    println!("active = {}", user1.active);
    println!("sing_in_count = {}", user1.sign_in_count);

    // change usernme
    user1.username = String::from("yuri kato");
    println!("username = {}", user1.username);
}

fn use_struct_2() {
    fn main() {
        let user1 = build_user(
            String::from("yoichiojima@gmail.com"),
            String::from("yoichi ojima")
        );
        println!("user1.username = {}", user1.username);

        // use an existing instance for a new instance
        let user2 = User {
            email: String::from("yurikato@gmail.com"),
            username: String::from("yuri kato"),
            ..user1
        };
        println!("user2.username = {}", user2.username);
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    main();
}

fn use_tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black.0 = {}", black.0);
    println!("origin.2 = {}", origin.2);
}