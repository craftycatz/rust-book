/*
    Functions are a way to group a series of statements together to perform a specific task.
    In rust functions are declared with the fn keyword.
*/


fn main() {
    println!("Hello, world!");

   another_function();

   a_function_that_takes_a_string("Hello from a function");

   let ten  = a_function_that_returns_an_integer(5, 5);
   println!("The value of ten is: {}", ten);

   println!("THe string literal is: {}", a_function_that_returns_a_string());
}

fn another_function() {
    println!("Another function.");
}

fn a_function_that_takes_a_string(string: &str) {
    println!("{}", string);
}

fn a_function_that_returns_an_integer(a: i32, b: i32) -> i32 {
    // In rust the 'return' keyword is not required.
    // The return value of the function is the last expression in the function.
    a + b
}

fn a_function_that_returns_a_string() -> String {
    return String::from("Hello from a function")
}