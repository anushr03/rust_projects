// Way 1: Easieset way to calculate Area
/*

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );
}

fn area (width: u32, height: u32) -> u32 {
    width * height
}
*/
//---------------------------------------------------------------
// Way 2: Using tuples
/*
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area(rect1)
    );

}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
 */

//---------------------------------------------------------------
//Way 3: Using Structs

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );

}

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

