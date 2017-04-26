fn main() {
    let resultado = suma(4, 5);
    println!("Hello, world! {:?}", "Rust");
    println!("Total {:?}", resultado);
}

/// # Examples
///
/// ```
/// let r: i32 = suma(4,5);
/// println!("{:?}", r);
/// 9
/// ```
pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}
