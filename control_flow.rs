fn main(){
    
    // handling Multiple Conditions with else if
    let character_level = 10;
    if character_level >= 5 && character_level < 10 {
        println!("You've earned a deer badge");
    } else if character_level >= 10 && character_level < 20 {
        println!("You've earned a bear badge");
    } else if character_level >= 20 && character_level < 30 {
        println!("You've earned a shark badge");
    } else {
        println!("You don't have any badges yet");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

}
