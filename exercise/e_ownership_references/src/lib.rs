pub fn inspect(arg: &str) {
    println!("Is {} plural? [{}]", arg, arg.ends_with("s"));
}

pub fn change(arg: &mut String) {
    if !arg.ends_with("s") {
        arg.insert_str(arg.len(), "s");
    }
}

pub fn eat(arg: String) -> bool {
    return arg.starts_with("b") && arg.contains("a");
}

pub fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}