fn main() {
    /*
     * Hay dos tipos de datos para las variables: "escalares" y "compuestos"
     *
     * Escalares:
     * Integers: numeros enteros
     * Floating point number: numeros flotantes
     * Boolean: valores booleanos
     * Characters: caracteres
     */

    /* Las variables unsigned son las que siempre son positivas ya que no llevan signo, las de tipo signed son las que pueden ser positivas y negativas. */
    let signed: u8 = 1;
    println!("The value of unsigned is: {signed}");

    let unsigned: i8 = -1;
    println!("The value of unsigned is: {unsigned}");

    /* Por defecto las variables tienen un tipo asignado de i32, enteros positivos de 32 bits */
    let i = 10;
    println!("The value of i is: {i}");

    /* Los numeros flotantes por defecto son asignados al tipo f64. */
    let f = 10.0;
    println!("The value of f is: {f}");

    /* El tipo de dato "char" (caracter) en Rust tiene un tamaño de 4 bytes y estan alineados al "Unicode Scalar Value", lo que significa que pueden representar mas que ASCII. */
    let z = '🔧';
    println!("The value of c is: {z}");

    /*
     * Compuestos:
     * Tuples: tuplas
     * Arrays: arreglos
     */

    /* Las tuplas son un tipo de dato que permite agrupar valores de diferentes tipos en una sola estructura. */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    /* Para acceder a los elementos individuales de la tupla podemos desestructurar creando distintas variables para los distintos valores */
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {x} {y} {z}");

    /* Tambien podemos acceder a sus valores mediante puntos */
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");

    /* Los arrays son un tipo de dato que permite alojar multiples datos del mismo tipo, en Rust, los arrays tienen que tener un largo determinado */
    let n = [1, 10, 20];

    /* Tambien se pueden inicializar para que tengan un mismo valor una cantidad determinada de veces, el siguiente ejemplo es lo mismo que escribir [3, 3, 3, 3, 3] */
    let s = [3; 5];

    /* Para acceder a los elementos de un array lo hacemos mediante un indice accediendo a las distintas posiciones del array */
    let first = n[0];

    /* Si intentamos acceder a un indice que excede al largo del array el compiladora paniqueara (panic compilation) devolviendo un error con el objetivo de proteger la memoría */
}
