#[test]
#[should_panic]
fn it_works() {
    assert_eq!("Hola", "Mundo");
}

#[test]
fn probar_suma() {
    assert_eq!(4, suma(2));
}

pub fn suma(a: i32) -> i32 {
    a + 2
}
