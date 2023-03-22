use rand::Rng;
/* Traemos el tipo "Ordering" de la librería estandar */
use std::cmp::Ordering;
/* Necesitamos la librería input/output (io) de la librería estandar "std", esta es la forma de obtenerla */
use std::io;

fn main() {
    println!("Guess the number!");

    /* La función thread_rng nos va a proveer de un generador de numeros aleatorio el cual es local al hilo de ejecución actual y "seeded" por el sistema operativo, el segundo metodo toma un rango como argumento y devuelve un numero aleatorio entre ese rango */
    let secret_number = rand::thread_rng().gen_range(1..=100);

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

    /* Haciendo shadowing de la variables guess, aplicamos ".trim" un metodo asociado a los string el cual remueve los espacios posibles al principio y final del string */
    /* El metodo ".parse()" en strings convierte un string a otro tipo de dato, para ello necesita saber a que tipo de dato lo vamos a convertir, para ello especificamos el tipo de la variable ":u32" */
    /* ".parse" devuelve un valor del tipo "Result", el cual ".expect()" maneja */

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    /* 1. "match" es una expresión que esta compuesta de "arms", un arm consiste en un patron con el cual coincidir, y el codigo que debería ejecutarse si el valor dado a match, coincide con el patron del arm */
    /* ".cmp" es un metodo que puede ser llamado en cualquier cosa que pueda ser comparado y toma como argumento la referencía de un valor. Luego compara el valor en el cual fue llamado contra el que fue pasado por argumento */
    /* ".cmp" retorna un valor del tipo Ordering, el cual es un enum que tiene tres variantes "Less", "Greater", "Equal" */
    /* Lo que sucede en el siguiente codigo es que se llama al metodo ".cmp" del valor "guess", y este metodo compara "guess" contra "secret_number", y luego devuelve un valor del tipo Ordening. Por ultimo, "match" toma ese Ordening devuelto por el ".cmp" y empieza a chequear ese valor contra los patrones de los arms. Si no coincide, pasa al siguiente. */
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
        }
    }

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // "_" es un valor que significa "matchea cualquier cosa", en este caso se lo pasamos a Err para decir que queremos coincidir con cualquier error
                break;
            }
        }
    }
}
