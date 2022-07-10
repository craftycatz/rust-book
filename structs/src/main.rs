/*
    Structs are a way to group data together to form a new type.
    This new type can have methods attached to it.
*/


// to create a struct you use the struct keyword.
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


// we can use tuple structs to create structs with named fields for clarity.
struct Rectangle{
    width: u32,
    height: u32,
}

// we can implement methods on structs.
// for that we use the impl syntax.
impl Rectangle{
    // inside the impl block we can define methods.
    // and reference the struct using self (like this in js).

    // as per convention contstructors are named with a 'new' prefix.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle{
            width,
            height,
        }
    }

    fn area(&self) -> u32{
        self.width * self.height
    }
}


fn main() {
    // to create a new instance of a struct you use the type and curly braces.
    let user1 = User{
        username: String::from("someone"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User{
        username: String::from("someone else"),
        email: String::from("test@test2.com"),
        //you can use the value from the other struct to initialize a new one with the .. syntax.
        ..user1
    };

    // to access the fields of a struct you use the dot syntax.
    println!("Username: {}, email: {}, sing in count: {}, active: {}", 
    user1.username, user1.email, user1.sign_in_count, user1.active);

    println!("Username: {}, email: {}, sing in count: {}, active: {}", 
    user2.username, user2.email, user2.sign_in_count, user2.active);

    /*fn area_rectangle(rectangle: &rectangle) -> u32{
        rectangle.width * rectangle.height
    }*/

    //calling the constructor of a struct. 
    let rect1 = Rectangle::new(30, 50);
    // calling the area method of the struct.
    println!("Area of rectangle: {}", rect1.area());
}
