fn main() {

    // String
    let greeting = "Hello there."; // greeting: &'static str

    let s = "foo
    bar";

    println!("lines {:?}", s);

    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    let s = "Hello".to_string();
    takes_slice(&s);

    // Index
    let s = "hello";

    for c in s.chars() {
        println!("{}, ", c);
    }

    //println!("The first letter of s is {}", s[0]); // ERROR!!!

    let char_h = s.chars().nth(0);

    println!(" Char! {:?}", char_h);

    // Slicing
    let dog = "忠犬ハチ公";
    let hachi = &dog[3..6];

    println!("Slicing {:?}", hachi);

    // Concatenacion
    let hello1 = "Hello ".to_string();
    let world1 = "world!";

    let hello_world = hello1 + world1;

    let hello = "Hello ".to_string();
    let world = "world!".to_string();

    let hello_world2 = hello + &world;

}

fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}
