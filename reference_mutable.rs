fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    // **
    // Multiple mutable references that avoid data race
    // **
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;

    // **
    // Using a mutable reference after inmutable refereces were dropped
    // **
    let r3 = &s; // no problem
    let r4 = &s; // no problem
    println!("{} and {}", r3, r4);
    // variables r1 and r2 will not be used after this point
    let r5 = &mut s; // no problem
    println!("{}", r5);

    // **
    // Avoiding a dangling reference
    // **    
    let string = no_dangle();

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}


