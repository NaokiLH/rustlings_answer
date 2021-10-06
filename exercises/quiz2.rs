// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    println!("blue");
    let t = ("red".to_string());
    let t = (String::from("hi"));
    let t = ("rust is fun!".to_owned());
    let t: String = ("nice weather".into());
    let t = (format!("Interpolation {}", "Station"));
    let t = (&String::from("abc")[0..1]);
    let t = ("  hello there ".trim());
    let t = ("Happy Monday!".to_string().replace("Mon", "Tues"));
    let t = ("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
