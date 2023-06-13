fn main() {
    let some_value: Option<i32> = Some(42);
    let unwrapped_value = some_value.unwrap();
    println!("Unwrapped value: {}", unwrapped_value); // Output: 42

    // let none_value: Option<i32> = None;
    // let unwrapped_value = none_value.unwrap(); // This will panic!
}
