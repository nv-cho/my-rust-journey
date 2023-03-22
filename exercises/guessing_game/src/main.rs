/* Necesitamos la librería input/output (io) de la librería estandar "std", esta es la forma de obtenerla */
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /* Lo que sucede en la siguiente linea es se esta creando una instancía vacia de un String, "String" es el tipo, "::" es la sintaxis que indica que new es una función asociada a el tipo String, una función asociada es una función implementada en el tipo */
    let mut guess = String::new();

    /* Esta linea lo que hace es acceder al metodo asociado "stdin" del elemento "io" que invocamos en la primer linea, si no la hubieramos invocado podriamos usarla de la siguiente forma "std::io::stdin" */
    io::stdin()
        /* Luego accedemos al metodo read_line de stdin, el cual es el encargado de manejar el input de usuario, recibirlo, y añadirlo al string que recibe como argumento */
        /* "&guess" es la forma de haceder a la referencía en memoria de la variable "guess", las referencías son inmutables por default, por ello es necesario agregar "mut" para hacerla mutable */
        .read_line(&mut guess)
        .expect("Failed to read line");

    /* ".read_line" es un metodo que devuelve una instancia del tipo "Result", el cual es un enum, un tipo de dato que puede ser una posible determinada cantidad de valores */
    /* En este caso "Result" puede ser del tipo "Ok" o "Err", como "Result" es una instancía, al escribir ".expect" estamos haciendo uso del metodo propío de "Result", si el valor de "Result" es del tipo "Err"  ".expect()" va a ser el encargado de crashear el programa e imprimir el mensaje que recibe como argumento. Si el valor es del tipo "Ok", ".expect" va a tomar el valor que "Ok" esta almacenando y lo va a devolver, en este caso, ese valor termina asignado a la variable "guess" */

    println!("You guessed: {guess}");
}
