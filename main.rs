fn main() {
    let a = 10;
    let b = &a;
    let c = &b;
    let d = b;
    let mut _e = &b;
    let f = &&100;

    _e = f;

    println!("Value of a: {:p}", &a);
    println!("Value of b: {:p}", b);
    println!("Value of c: {:p}", c);
    println!("Value of d: {:p}", d);

    println!("Value of e: {:p}", _e);
    println!("Value of f: {:p}", f);
    println!("Address of 100: {:p}", &(**f));

    println!("Value of e: {:p}", *_e);
    println!("Value of f: {:p}", *f);
    println!("Address of 100: {:p}", &(**_e));
}
