fn main() {
    /* Asi es como se define una tupla. Las tuplas son estructuras que permiten albergar multiples valores y estos pueden ser de distinto tipo. */
    let point: (i64, i64, i64) = (0, 0, 0);

    /* Podemos acceder a los valores de las tuplas de esta manera, accediendo a los elementos de forma individual */
    let x = point.0;
    let y = point.1;
    let z = point.2;

    /* O desestructurando los elementos de la tupla */
    let (x, y, z) = point;

    /* Se utiliza _ para indicar que ese valor no es utilizada */
    let (x, y, _) = point;

    /* Para volver mutables a los valores de una tupla, añadimos la palabra mut */
    let mut point_2: (i64, i32, u64) = (0, 0, 0);

    point.0 = 17;
    point.1 = 42;
    point.2 = 90;

    /* Tuplas sin ningun valor, son llamadas units y no pueden contener ningun valor, y aseguran que no contienen ningun valor */
    let unit: () = ();
}

/* Las funciones unit se pueden utilizar para especificar como para los casos en los cuales no hay ningun valor retornado */
fn unit_example() -> () {
    println!("No return");

    let println_return: () = println!("Nooooo returnnnnn");
}
