fn main() {
    // Default imutable
    let text: String = String::from("Hello");
    let len: usize = reference(&text);
    println!("The length of '{}' is {}.", text, len);
    // Passed imutable reference
    // Restriction:
    // 1. only allowed to borrowed 1 mutable at a time
    // 2. can borrow as mutable if already borrowed as immutable
    ///// we can, if the previous scope already ends
    // Reason: prevent data races at compilation time
    let mut text1: String = String::from("Hello");
    change_refrence(&mut text1);
    println!("New text1 {}", text1);
}



fn reference(s: &String) -> usize {
    let length: usize = s.len();
    length
}

fn change_refrence(s: &mut String) {
    s.push_str(", world");
}