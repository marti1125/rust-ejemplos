fn main(){

    let caballo = '♞';  // char

    // Booleanos
    let t = true;
    let f :bool = false;

    // Tipos Numericos
    let n1 = 42; // de tipo i32
    let float = 1.5; // de tipo f64

    let nombres = ["Graydon", "Brian", "Niko"];

    println!("la cantidad de elementos es {:?}",nombres.len());
    println!("el segundo nombre es: {:?}", nombres[1]);

    // casteando un variable
    let total = n1 as f64 + float;
    println!("Total de {:?} + {:?} es: {:?}", n1, float, total);

    // SLICES
    let elementos = [0, 1, 2, 3, 4];
    let medio = &elementos[1..4]; // un slice de 1,2,3
    let completo = &elementos[..]; // un slice contiene todo sus elementos

    println!("medio: {:?}", medio);
    println!("completo: {:?}", completo);

    // TUPLES
    let x = (1, "hola"); //let x: (i32, &str) = (1, "hello");

    let (x, y, z) = (1, 2, 3);
    println!("x es {}", x);

    // TUPLE INDEXING
    let tupla = (1, 2, 3);

    let a = tupla.0;
    let b = tupla.1;
    let c = tupla.2;

    println!("a es {}", a);

}
