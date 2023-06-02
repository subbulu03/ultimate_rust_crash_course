/**
 * One of the best and simplest examples of a trait in Rust is the `Display` trait.
 * The `Display` trait allows objects to be formatted as a string
 * when using the `println!` macro or other formatting macros.
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
