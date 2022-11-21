fn main() {
    let mut message = String::from("Hello");
    let message_3 = message.clone();

    // let slice = &message[2..=4];

    message.clear();

    println!("message: {}", message);
    println!("message_3: {}", message_3);
}
