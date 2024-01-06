fn function_that_might_fail() -> Result<String, std::io::Error> {
    let flag = false;
    if flag {
        Ok(String::from("Success."))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failure."))

    }
}

fn main() {
    let result = function_that_might_fail().expect("the function returned an error");
    println!("Result: {}", result);
}