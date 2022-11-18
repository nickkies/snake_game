fn main() {
    let a = 10;
    let b = a;
    let _c = 15; // copy
    let sum = add(a, b);

    println!("{}", sum);

    let message = String::from("Hello");
    let message_2 = message; // move

    // error
    // println!("{}", message);
    println!("{}", message_2);
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}
