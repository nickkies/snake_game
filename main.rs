fn main() {
    let message = String::from("Hello");
    print_message(message);

    // error
    // println!("{}", message);
}

fn print_message(a: String) {
    println!("{}", a);
} // drop
