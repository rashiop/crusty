#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect_small = Rectangle {
        width: 10,
        height: 10,
    };


    let rect_large = Rectangle {
        width: 100,
        height: 70,
    };

    println!("Can rect hold rect_small? {}", rect.can_hold(&rect_small));
    println!("Can rect hold rect_large? {}", rect.can_hold(&rect_large));


    dbg!(&rect);
}

