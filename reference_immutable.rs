fn main() {
    let s1 = String::from("hello");

    // &s1 is a reference to the value of s1
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// The function accepts a reference to a String value
fn calculate_length(s: &String) -> usize {
    s.len()
}
