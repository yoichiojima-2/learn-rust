fn main() {
    // use_string();
    // stack_version();
    using_clone();
    copy_stack();
    see_how_scope_works();
}

fn see_how_scope_works() {
    let s = String::from("hello");
    println!("s = {s}");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
}


fn using_clone() {
    let s1 = String::from("s1");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}", s1 = s1, s2 = s2);
}

fn copy_stack() {
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}", x = x, y = y);
}

// fn stack_version() {
//     // use_string();
//     let x = 5;
//     let y = x;
//     println!("x = {x}, y = {y}", x = x, y = y);
// }

// fn use_string() {
//     let mut s: String = String::from("hello");
//     s.push_str(", Yoichi!");
//     println!("s = {s}");
// }