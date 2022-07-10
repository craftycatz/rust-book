fn main() {
    let x = 5;
    // in rust we can use the 'if' keyword to create a conditional statement.
    // we can also use the else keyword to create an else statement.

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    } // end if

    // we can also use the if  keyword in an assignment statement.
    let y = if x == 5 { 10 } else { 15 }; // end if

    println!("The value of y is: {}", y);

    // we can also use the 'match' keyword to create a switch statement.
    let number = 6;
    match number {
        1 => println!("one!"),
        2 => println!("two!"),
        3 => println!("three!"),
        4 => println!("four!"),
        5 => println!("five!"),
        _ => println!("I don't know!"),
    } // end match

    // we can use the loop keyword to create a loop statement.
    // the loop keyword will execute the code inside the loop until the condition is false.
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
        println!("The value of counter is: {}", counter);
    }; // end loop

    // we can use the while keyword to create a while loop.
    // the while keyword will execute the code inside the loop until the condition is false.
        let mut number = 3;
    
        while number != 0 {
            println!("{number}!");
    
            number -= 1;
        }
    
        println!("LIFTOFF!!!");


        // we can use the for keyword to create a for loop.
        // the for keyword will execute the code inside the loop until the condition is false.
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is: {}", element);
        } // end for

    
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
}
