pub fn inspect(x: &String) {
    if (*x).ends_with("s") {
        println!("Plural")
    } else {
        println!("Singular");
    }
}

pub fn change(x: &mut String) {
    if !(*x).ends_with("s") {
        (*x).push_str("s");
    }
}

pub fn eat(x: String) -> bool {
    x.starts_with("b") && x.ends_with("a")
}

pub fn bedazzle(x: &mut String) {
    *x = String::from("sparkly");
}
