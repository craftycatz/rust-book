/*
    In Rust there exists the concept of ownership.
    Ownership is the concept of who is responsible for the lifetime of a value.
    The value is destroyed when the owner goes out of scope.
    for example:
    fn main(){
        let s = String::from("hello");
        takes_ownership(s);
        // s is no longer valid here
    }
    fn takes_ownership(some_string: String){
        println!("{}", some_string);
    }
    The ownership of the string is passed to the function.
    The function then takes ownership of the string and destroys it when it is done with it.   

    There are three basic ownership rules:
        1.Each value in Rust has a variable that is called its owner.
        2.There can only be one owner at a time.
        3.When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    fn a(){
        // the size of x and y is known at compile time
        // therefore these values live on the stack
        let _x: &str = "hello";
        let _y: i32 = 42;
        b();
    }

    fn b(){
        //the size of z is not known at compile time
        // therefore this value lives on the heap
        // z is a pointer to the value on the heap
        let _z: String = String::from("world");
        // rust automatically allocates memory for z, in c you would have to call malloc(sizeof(z)) to allocate memory for z
        // rust also automatically deallocates memory for z when it goes out of scope so you dont have to worry about calling free(z)
        // this prevents memory unsafety issues such as use after free, double free etc.
    }
    // pushing to the stack is faster than allocation on the heap
    // accessing a value on the stack is faster than accessing a value on the heap.

    {// s is not valid because it is not delared yet.
        let s = String::from("i  live on the heap");// the memory for s is allocated on the heap
        // s is valid because it is declared in this scope.
        println!("{}", s);
    }//this scope is now over and s is no longer valid (the memory on the heap is deallocated).

    let _x = 5;
    let _y = _x; // Copy the value of x into y 
    // values that live on the stack are copied!
    // values that live on the heap are moved!
    let _s1 = String::from("hello");
    let _s2 = _s1;// s1 is moved (in c++ this could be achived using std::move()) to s2 and s1 is no longer valid.
    //comment out the println! and run the code to see the error.
    //println!("{}", _s1); // this will cause an error because s1 is no longer valid.

    let s3 = String::from("hello");
    let s4 = s3.clone();// s3 is cloned to s4.
    println!("s3 = {}, s4 = {}", s3, s4);

    fn takes_ownership(some_string: String){
        // passing a variable that lives on the heap into a function passes the ownership of the variable to the function
        println!("I own the string which contains: {}", some_string);
    }   

    fn makes_copy(some_integer: i32){
        // passing a variable that live on the stack into a function makes a copy of the variable
        println!("I only have a copy of: {}", some_integer);
    }
    takes_ownership(s3);
    // s3 is no longer valid here
    //println!("{}", s3); // this will cause an error because s3 is no longer valid.
    let number = 3;
    makes_copy(number);
    // number is still valid here

    fn gives_ownership() -> String{
        // this function will return a string
        let some_string = String::from("hello");
        //the ownership of the string is passed out of the function
        some_string
    }

    let s69 = gives_ownership();
    // s69 is valid here
    println!("{}", s69);

    fn takes_and_gives_back(a_string: String) -> String{
        // this function takes a string and returns a string
        println!("I own the string which contains: {}, until i return it", a_string);
        a_string
    }
    let s70 = takes_and_gives_back(s69);
    // s70 is valid here
    println!("{}", s70);

    // if you want to pass a read only reference to a variable you can use the & syntax.
    // that way the function can only read the variable but not modify it, 
    // the ownership of the variable is not passed to the function.
    fn print_string(a_string: &String){
        println!("I do not own the string which contains: {}", a_string);
    }

    let test_s = String::from("I am a test string");
    print_string(&test_s);
    // test_s is still valid here
    println!("{}", test_s);
    // you can have as many immutable references as you want to a variable.

    // if you want to pass a mutable reference to a variable you can use the &mut syntax.
    // that way the function can modify the variable but not create a new one, owership of the variable is not passed to the function.
    fn mutate_string(a_string: &mut String){
        a_string.push_str(", and I am mutable");
    }
    // the variable must be mutable to be passed as a mutable reference.
    let mut test_s2 = String::from("I am a test string");
    mutate_string(&mut test_s2);
    // test_s2 is still valid here
    println!("{}", test_s2);
    // test_2 is modified here
    // you can have only one mutable reference to a variable.
    // you can also have only one mutable reference and no other immutable references to a variable.

    /*
        There are basic rules of references
            1. At any given time there can only be one mutable reference to a variable or 
            any number of immutable references.
            2. References must always be valid. Dangling references are not allowed by the compiler!.
    */


    // SLICES
    // slices are a pointer to a section of an array.
    // slices are used to give the ownership of a section of an array to a function.
 
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);

    // slices can also be created from a string.
    let slice_str = "hello world";
    let first_word = &slice_str[0..5];
    println!("{:?}", first_word);
}
