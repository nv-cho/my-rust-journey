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
}
