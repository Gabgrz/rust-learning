fn main (){
    /* 
    Ownership rules:
    - Each value in Rust has an owner
    - There can only be one owner at a time
    - When the owner goes out of scope, the value will be dropped
     */

    // The following variables are only valid within this scope (block)

    // "hello" is a string literal so cannot be mutated
    let s_literal = "hello";

    // Creating an immutable String type from a string literal
    let s_inmut = String::from("hello");

    // Creating a mutable String type from a string literal
    // The memory for the String type must be requested from the memory allocator at runtime.
    let mut s_mut = String::from("hello");
    s_mut.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s_mut); // This will print `hello, world!`

    // 
    // Variables and Data Interacting with Move
    //

    let x = 5;
    let y = x; // The value of x gets copied and bound to y. Both 5 values get pushed into the stack. Copies in the stack are cheap and quick.

    // However, this behavior is different with Strings
    let s1 = String::from("hello");
    let s2 = s1;
    // The string data (pointer, lenght and capacity) of s1 is MOVED  to s2 Data on the heap that the pointer refers to is NOT copied. Copying strings is memory-expensive.
    // s1 becomes invalid

    // To clone/copy, use the clone method instead
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    //
    // Stack-Only Data: Copy
    //
    
    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait
    // Any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy.
    /* 
    Types that implement copy
    - All the integer types, such as u32.
    - The Boolean type, bool, with values true and false.
    - All the floating-point types, such as f64.
    - The character type, char.
    - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.        
    */ 
    
} 
// this scope is now over, and our variables are no longer valid
// Rust calls drop automatically at the closing curly bracket.
// This pattern is called Resource Acquisition Is Initialization (RAII)
