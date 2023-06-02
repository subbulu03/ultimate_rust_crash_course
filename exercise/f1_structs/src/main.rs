// an attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Venkat");
    let age = 32;
    let person = Person{ name, age };
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("{:?}", person);
}
