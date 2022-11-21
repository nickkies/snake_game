fn main() {
    let _num = 32; // stack
    let num_2 = Box::new(100); // heap

    println!("{}", num_2);
}
