fn main() {
    
    // Immutable variable
    let x = 1;
    
    // Immutable variable with explicit type
    let z: bool = true;    
    
    // Mutable variable
    let mut y = 2;
    
    // Assign new value to 'y'
    y = 3;
    
    // Constant
    const PI: f64 = 3.14159;
    
    // Print a string
    println!("Hello, world!");
    
    // Print values assigned to 'x' and 'y'; requires a string literal to format with
    println!("x: {}\ny: {}", x, y);
    
    // Print sum of x + y; requires a string literal to format with
    println!("x + y = {}", x + y);

    // Define a function that takes two integers and returns their sum
    fn add(x: i32, y: i32) -> i32 {
        // Return values are implicit; no need for a return keyword
        x + y
    }
    
    // Call the function with two arguments
    let sum = add(x, y);
    println!("Sum result: {}", sum);
    
    // Use an if/else expression to check a condition
    if sum < 0 {
        println!("The number is negative");
    } else if sum > 0 {
        println!("The number is positive");
    } else {
        println!("The number is zero");
    }
    
    // Use a match expression to match on different values or patterns
    let color = "yellow";
    match color {
        "red" => println!("The color is red"),
        "green" => println!("The color is green"),
        "blue" => println!("The color is blue"),
        "yellow" => println!("The color is yellow"),
        _ => println!("The color is unknown"), // _ matches any other value
    }
    
    // Use a loop to repeat a block of code indefinitely
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter == 3 {
            break; // Exit the loop
        }
    }
    
    // Use a while loop to repeat a block of code while a condition is true
    let mut number = 3;
    while number != 0 {
        println!("Number: {}", number);
        number -= 1;
    }
    
    // Use a for loop to iterate over a collection
    let array = [1, 2, 3];
    for element in array.iter() {
        println!("Element: {}", element);
    }
    
    // Use a for loop to reverse iterate over a range
    for number in (1..=5).rev() {
        println!("Number: {}", number);
    }
    
}
