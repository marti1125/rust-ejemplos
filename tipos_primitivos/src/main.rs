fn main(){

    // Char
    let caballo = '♞';

    println!("Caballo {:?}", caballo);

    // Booleanos
    let t = true;
    println!("boolean {:?}", t);
    let f :bool = false;
    println!("boolean f:bool {:?}", f);

    // Tipos Numericos
    let n1 = 42; // de tipo i32
    let float = 1.5; // de tipo f64

    let nombres = ["Graydon", "Brian", "Niko"];

    println!("la cantidad de elementos es {:?}",nombres.len());
    println!("el segundo nombre es: {:?}", nombres[1]);

    // casteando un variable
    let total = n1 as f64 + float;
    println!("Total de {:?} + {:?} es: {:?}", n1, float, total);

    let mut z = 2;

    z = 8;

    //Array
    let mut m = [1,3,5];

    m[0] = 1;
    m[1] = 6;

    println!("Array {:?}", m);

    // SLICES
    let elementos = [0, 1, 2, 3, 4];
    let medio = &elementos[1..4]; // un slice de 1,2,3
    let completo = &elementos[..]; // un slice contiene todo sus elementos

    println!("medio: {:?}", medio);
    println!("completo: {:?}", completo);

    // TUPLES
    let x = (1, "hola"); //let x: (i32, &str) = (1, "hello");
    println!("Una tupla simple {:?}", x);

    let (x, y, z) = (1, 2, 3);
    println!("x es {}", x);
    println!("y es {}", y);
    println!("z es {}", z);

    // TUPLE INDEXING
    let tupla = (1, 2, 3);

    let a = tupla.0;
    let b = tupla.1;
    let c = tupla.2;

    println!("a es {}", a);
    println!("b es {}", b);
    println!("c es {}", c);

}
