fn main() {
  ownership();
}

fn ownership() {
    //// Purpose: invalidate memory a.k.a move
    //// Area for moving:
    // 1. non simple data type (simple => copy = OK)
    // 2. passing var to function
    // 3. give ownership aka return value
    
    //// Code:
    // 1. Non Simple variable
    let s1: String = String::from("Hallo");
    println!("{}, world! - 1", s1);
    // Error: the s1 is already moved to s2
    // let s2: String = s1;
    // println!("{}, world!", s1);
    let s2: String = s1.clone();
    println!("{}, {}, world! - 2", s1, s2);

    // 2. Passing var to function
    let s3: String = String::from("Udra");
    // Ownership move to text on take_ownership
    take_ownership(s3);
    // println("checking ownership of: {}", s3);
    
    //// 3. Give ownership
    let cambodian_cat = give_ownership();
    println!("Cambodian cat: {}", cambodian_cat); 
}

fn take_ownership(text: String) {
    println!("take ownership of: {}", text)
}

fn give_ownership() -> String {
    let cat: String = String::from("Bongki");
    cat
}