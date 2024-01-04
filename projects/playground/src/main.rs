use std::io;


fn main() {
    try_variables();
    try_if_statements();
    try_types();
    try_repetition();
}

fn try_repetition() {
    loop_statement(1);
    nested_loop();
    while_loop();
    for_loop();
}

fn try_variables() {
    declear_mutable_variable();
    declear_constant();
    shadowing();
}

fn try_types() {
    types();
    access_array_element();
    function_takes_parameters(5, 6);
}

fn try_if_statements() {
    if_statement("success");
    if_statement("error");
}

fn declear_mutable_variable() {
    let mut x = 5;
    print("x", x);
    x = 6;
    print("x", x);
}

fn print(label: &str, value: i32) {
    println!("{label}: {value}");
}

fn declear_constant() {
    const X: i32 = 100;
    println!("X = {X}");
}

fn shadowing() {
    let status = "outside of the scope";
    {
        let status = "inside of the scope";
        println!("status = {status}");
    }
    println!("status = {status}");
}

fn types() {
    let x: i32 = 5;
    println!("x = {x}");

    let y: char = 'a';
    println!("y = {y}");

    let s: &str = "hello";
    println!("s = {s}");

    let t: (i32, i32, i32) = (100, 200, 300);
    println!("t = {}, {}, {}", t.0, t.1, t.2);

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for i in a {
        println!("i = {i}");
    }
}

fn access_array_element() {
    let a = [10, 20, 30, 40, 50];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    println!("The value of the element at index {} is {}", index, a[index]);
}

fn function_takes_parameters(x: i32, y: i32) {
    println!("x = {x}, y = {y}");
}

fn if_statement(status: &str) {
   if status == "success" {
        println!("success :)");
   } else {
        println!("failed :(")
   }
}

fn loop_statement(index_to: usize) {
    let a = ["Apple", "Orange", "Lemon"];
    let mut index = 0;
    let result = loop {
        if index == index_to {
            break a[index];
        }
        index += 1;
    };
    println!("result = {result}");
}

fn nested_loop() {
    let mut x = 0;
    let mut y = 0;
    'outer_loop: loop {
        'inner_loop: loop {
            println!("x = {x}, y = {y}");
            if x == 5 {
                break 'outer_loop;
            }
            if y == 5 {
                break 'inner_loop;
            }
            y += 1;
        }
        x += 1;
    }
}

fn while_loop() {
    let mut x = 0;
    while x < 10 {
        println!("x = {x}");
        x += 1;
    }
}

fn for_loop() {
    let a = ["Apple", "Orange", "Banana"];
    for i in a {
        println!("i = {i}");
    }
}
