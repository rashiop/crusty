fn main() {

    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect)
    );
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}