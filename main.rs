fn main() {
    let float_num: f32 = 3.14;
    let float_num_2 = 3.2;

    let tup = (float_num + float_num_2, "Hi", "there");
    println!("{}", tup.1);

    let (_, _, c) = tup;
    println!("{}", c);

    let x: [i32; 4] = [1, 5, 6, 7];
    println!("{}", x[2]);

    let y = [2; 6]; // [2, 2, 2, 2, 2, 2]
    println!("{}", y[5]);
}
