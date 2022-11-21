fn main() {
    let a = 10;
    let b = &a;
    let c = &b;

    println!("{}", a == *b);
    println!("{}", a == **c);
}
