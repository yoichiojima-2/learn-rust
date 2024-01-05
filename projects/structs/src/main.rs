fn main() {
    use_struct();
}

fn use_struct() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let me = User {
        email: String::from("yoichiojima@gmail.com"),
        username: String::from("yoichiojima"),
        active: true,
        sign_in_count: 1,
    };

    println!("username = {}", me.username);
    println!("email = {}", me.email);
    println!("active = {}", me.active);
    println!("sing_in_count = {}", me.sign_in_count);
}
