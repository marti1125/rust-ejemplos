fn main() {
    // WHILE
    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    // FOR
    for x in 0..10 {
        println!("{}", x); // x: i32
    }
}
