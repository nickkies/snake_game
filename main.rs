fn main() {
    let mut message = String::from("Hello");
    let message_2 = &mut message;

    message_2.push_str(" World");

    println!("{}", message_2); // mutable borrow
    println!("{}", message); // immutable borrow

    // println!("{}", message_2); // ㅠㅠ
}
