fn main() {
    let custom_num = 98_000;
    // hexadecimal
    // 16^1 * 15 + 16^0 * 10 = 250
    let hex_num = 0xfa;
    let bin_num = 0b0010_1011;
    // UTF table
    // 0x41
    // 16^1 * 4 + 16^0 * 1 = 65
    let byte_num = b'A';

    println!("{}", custom_num);
    println!("{}", hex_num);
    println!("{}", bin_num);
    println!("{}", byte_num);
}
