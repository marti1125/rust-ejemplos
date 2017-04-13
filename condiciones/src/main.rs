fn main() {

    let x = 5;

    if x == 5 {
        println!("¡x es cinco!");
    } else if x == 6 {
        println!("¡x es seis!");
    } else {
        println!("x no es cinco o seis :(");
    }

    let x = 5;
    let y = if x == 5 {
        10
    } else {
        15
    };
    println!("el valor de y es: {:?}",y);

}
