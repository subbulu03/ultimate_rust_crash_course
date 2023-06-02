/**
 * One of the best and simplest examples of a trait in Rust is the `Display` trait.
 * The `Display` trait allows objects to be formatted as a string
 * when using the `println!` macro or other formatting macros.
 */
/**
 * In this example, we define a Point struct with x and y coordinates. We want to be able to print a Point using the println! macro, so we implement the Display trait for the Point struct.

The Display trait requires implementing the fmt method, which formats the object as a string. In this case, we use the write! macro to write the formatted string representation of the Point to the provided formatter (f). The resulting fmt::Result indicates whether the formatting was successful or not.

In the main function, we create a Point instance p with coordinates (5, 10), and then we use the println! macro to print the formatted string representation of p using the {} placeholder. The output will be:
 */
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point{ x: 5, y: 6 };

    println!("The point is: {}", p);
}
