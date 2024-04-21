use rand::Rng;
fn main() {
    variables();
    control_flow();
    loop_flow();
}

fn control_flow() {
    let score = rand::thread_rng().gen_range(1..=100);
    println!("\n\nNormal day at school\nTeacher: your score is {}", score);
    println!("Mom opens your score card then paused for a moment...");
    if score < 50 {
        println!("Congrats, you have been disowned!");
    } else if score < 70 {
        println!("Mom: You are a disgrace to our family, I'll give you a reason to cry");
    } else if score < 90 {
        println!("Mom: Cousin Steven got 100");
    } else if score < 100 {
        println!("Mom: All game and no study");
    } else {
        println!("Mom: Eat some mango dear");
    }

    let number = if true { 5 } else { 6 };
    println!("Ternary {number}");
}

fn loop_flow() {
    let mut counter = 0;

    let mut result = loop {
        if counter >= 5 {
            break counter;
        }

        counter += 1;
    };

    println!("Result 1: {result}");


    while result < 10 {
        result += 1;
    }
    println!("Result 2: {result}");

    let nicks = ["Pupu", "Sammy", "Samudra", "Samucil", "Alpupu", "Pupucino", "Baby", "Sweety", "Cutie"];
    for nick in nicks.iter() {
        println!("Udra is also known as {nick}");
    }

    for num in 0..4 {
        println!("Just a normal loop {num}");
    }
}

fn variables() {
     let mut x = 10;
    println!("x is {}", x);
    x = 72;
    println!("x is {}", x);


    // constant cant be set mutable
    const CAT_COUNT: i8 = 8;
    const FUR_COUNT: i64 = 1_000_000_000_000_000;
    println!("There are {} cats with total furs {}", CAT_COUNT, FUR_COUNT);
    // boolean
    let is_smart: bool = true;
    println!("Udra is smart {}", is_smart);
    // char
    let udra_first_char: char = 'U';
    println!("Udra first char is {}", udra_first_char);


    // Compound:
    // tuple
    let udra_the_cat = ("Udra", ["Pupu", "Sammy", "Samudra", "Samucil", "Alpupu", "Pupucino"], 2017, 3.5, "Female", true);
    let (name, nicknames, birth_year, weight, gender, is_smart) = udra_the_cat;
    println!("{} is a {} cat weighted {}kgs, born at {}. Everyone know that she is {}ly smart. You can call her {}. However she wont answered if you are not me haha", name, gender, weight, birth_year, is_smart, nicknames.join(", "));
    println!("First tuple: {}\n\n", udra_the_cat.0);
}
