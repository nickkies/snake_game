fn main() {
    let message = String::from("Hello");
    let message = extend_message(message);

    let age = 300;
    extend_age(age);
    print!("{}", age);

    println!("{}", message);
}

fn extend_message(mut a: String) -> String {
    a.push_str(" World");
    a
}

fn extend_age(mut _a: u32) {
    _a += 100;
}
