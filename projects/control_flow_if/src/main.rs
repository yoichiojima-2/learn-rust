fn main() {
    simple_if();
    not_in_if();
    expression_if();
    if_in_let();
}

fn simple_if() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn not_in_if() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero.");
    }
}

fn expression_if() {
    let number = 8;

    if number % 4 == 0 {
        println!("the number is divisible by 4");
    } else if number % 3 == 0 {
        println!("the number is divisible by 3");
    } else if number % 2 == 0 {
        println!("the number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2.");
    }
}

fn if_in_let() {
    let condition = true;

    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}