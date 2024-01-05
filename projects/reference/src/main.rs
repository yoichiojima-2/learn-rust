fn main() {
    use_ref();
    use_slices();
    use_slice_2();
    use_slice_3();
    use_slice_4();
    use_slice_5();
}

fn use_ref() {

    fn main() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    main();
}

fn use_slices() {

    fn main() {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        println!("hello = {hello}, world = {world}");
    }

    main();
}

fn use_slice_2() {

    fn main() {
        let s = String::from("hello");
        let slice_1 = &s[..3];
        let slice_2 = &s[2..];
        println!("slice_1 = {slice_1}, slice_2 = {slice_2}");
    }

    main();
}

fn use_slice_3() {

    fn main() {
        let s = String::from("hello");
        let len = s.len();
        let slice_1 = &s[..len - 3];
        println!("slice_1 = {slice_1}");
    }

    main();
}

fn use_slice_4() {

    fn main() {
        let s = String::from("hello world");
        let word = first_word(&s);
        println!("the first word is: {word}");
    }

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]
            }
        }
        &s[..]
    }

    main();
}

fn use_slice_5() {
    fn main() {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        for i in slice {
            println!("i = {i}");
        }
    }
    main();
}
