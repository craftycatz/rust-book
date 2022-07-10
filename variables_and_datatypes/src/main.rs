fn main() {
    /*
        Variables are declared with the keyword `let` and are immutable by default.
        that means they can't be changed once they are assigned a value.
        Rust has a special syntax for declaring variables that are mutable.
        This is called a mutable variable.
        The keyword `mut` is used to declare a mutable variable.
    */
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    /*
        The value of a constant must be known at compile time.
        therefore, we must specify the type of the constant.
        Rust can not infer the type of the constant, as this happens at runtime.
        furhtermore we can't assign the result of a function to a constant.
        Rust will not allow you to do this, because the return value of the function is unknown at compile time.
    */
    const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    /*
        Shadowing in Rust allows us to use the same name for different variables.
        This is useful when we want to reuse the same name for different variables.
        The compiler will use the last variable that is declared until its scope ends.
    */
    {
        //inner scope of the block
        // here we shadow the outer scope variable x with a new variable called x
        let x: i32 = x * 100;
        println!("The value of  'shadow x' is: {}", x);
    }
    // here we can use the original x again as as the shadowed variable is out of scope.
    println!("The value of x in global scope is: {}", x);

    // Data types in Rust
    // Rust has a number of built-in data types.
    // These are:
    // - integers: i8, i16, i32, i64, i128, isize
    // - unsigned integers: u8, u16, u32, u64, u128, usize
    // - floating point numbers: f32, f64
    // - characters: char
    // - boolean: bool
    // - tuples: (i32, f64, u8)
    // - arrays: [i32; 10]
    // - slices: &[i32]
    // - strings: String
    // - vectors: Vec<i32>
    // - pointers: *const i32, *mut i32
    // - references: &i32, &mut i32
    // - raw pointers: *const i32, *mut i32
    // - functions: fn(i32) -> i32
    // - enums: enum Color { Red, Green, Blue }
    // - structs: struct Point { x: i32, y: i32 }
    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }
    
    let mut integer: i32 = 5;
    let mut unsigned_integer: u32 = 5;
    let mut floating_point_number: f32 = 5.0;
    let mut character: char = 'a';
    let mut boolean: bool = true;
    let mut tuple: (i32, f64, u8) = (500, 6.4, 1);
    let mut array: [i32; 10] = [0; 10];
    let mut slice: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut string: String = String::from("Hello");
    let mut vector: Vec<i32> = Vec::new();
    let mut pointer: *const i32 = &integer;
    let mut reference: &i32 = &integer;

    println!("\ndata types in Rust:");
    print_type_of(&integer);
    print_type_of(&unsigned_integer);
    print_type_of(&floating_point_number);
    print_type_of(&character);
    print_type_of(&boolean);
    print_type_of(&tuple);
    print_type_of(&array);
    print_type_of(&slice);
    print_type_of(&string);
    print_type_of(&vector);
    print_type_of(&pointer);
    print_type_of(&reference);


}
