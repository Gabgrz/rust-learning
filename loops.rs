#![allow(unreachable_code)]

fn main() {

    // returning Values from Loops
    let mut match_counter = 0;
    
    let match_counter = loop {
        match_counter += 1;

        if match_counter == 10 {
            break match_counter; // 
        }
    };
    println!("You have reached the maximum daily amount of matches: {match_counter}");

    // loop Labels to Disambiguate Between Multiple Loops
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");    

}
