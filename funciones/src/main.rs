fn main() {
    saludo("Mozilla".to_string());
    println!("Suma: {:?}", sumar(100,898));
    println!("Potencia: {:?}", potencia(5,5));
    let total = sumar(100,898) + potencia(5,5);
    println!("Total: {:?}", total);
}

fn saludo(name :String) {
    println!("Hello, {:?}", name)
}

fn sumar(x :i32, y :i32) -> i32 {
    x + y
}

pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}

fn resta(a: i32, b: i32) {
    let r: i32 = a - b;
    println!("Result {:?}", r);
}

fn potencia(x :i32, y :i32) -> i32 {
    let mut result = 1;
    for i in 1..y+1 {
        result = result * x;
        println!("Respuesta, {:?}", result);
    }
    result
}
