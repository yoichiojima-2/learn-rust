fn main() {
    // declaring basic types
    let num: i32 = 5;
    println!("{num} should be numeric.");

    // charactor
    let c: char = 'c';
    println!("{c} should be a charactor");

    // tuple
    // pack variables into tule
    let t: (i32, char) = (num, c);
    // unpacking variables from tuple`
    let a_in_tuple = t.0;
    let b_in_tuple = t.1;
    // printing
    println!("{a_in_tuple} should be numeric");
    println!("{b_in_tuple} should be charactor");

    // array;
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first= a[0];
    let second = a[1];
    println!("first: {first} should be 1.");
    println!("second: {second} should be 2.");
}