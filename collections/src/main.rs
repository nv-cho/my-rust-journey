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

    point_2.0 = 17;
    point_2.1 = 42;
    point_2.2 = 90;

    /* Tuplas sin ningun valor, son llamadas units y no pueden contener ningun valor, y aseguran que no contienen ningun valor */
    let unit: () = ();

    /* Esta es la forma de declarar estructuras */
    struct Point {
        x: i64,
        y: i64,
        z: i64,
    }

    fn new_point(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z } // equals to: Point { x: x, y: y, z: z }
    }

    let point = Point { x: 1, y: 2, z: 3 };

    /* Puedo acceder a los valores de forma individual o desestructurandolos */
    let x = point.x;
    let Point { x, y, z } = point;

    let mut mut_point = Point { x: 5, y: 3, z: 7 };
    mut_point.x = 10;
}

/* Las funciones unit se pueden utilizar para especificar como para los casos en los cuales no hay ningun valor retornado */
fn unit_example() -> () {
    println!("No return");

    let println_return: () = println!("Nooooo returnnnnn");
}
