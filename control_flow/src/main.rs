fn main() {
    let number = 3;

    /* Los bloques de codigo asociados con condiciones pueden ser llamado "arms" */
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /* if/else if/else son expresiones, es decir, evaluan hacía un valor. Eso quiere decir que podemos usarla para asignar. Ambos valores posibles del if y el else deben ser del mismo tipo */
    let condition = true;
    let number = if condition { 5 } else { 6 };

    /* El bloque loop se ejecuta una y otra vez para siempre o hasta que le de una indicación explicita para que pare */
    let mut counter = 0;

    /* Tambíen podemos usar un loop como una expresión. */
    let result = loop {
        counter += 1;
        println!("again!");

        if counter == 10 {
            /* Si queremos detener el loop utilizamos "break", y si queremos devolver algo al finalizar el loop, colocamos el valor a devolver luego de "break". */
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    /* Se puede asignar un label a un loop y luego indicar con ese mismo label si queremos interrumpir el loop, de esta forma podemos crear codigos como el siguiente */
    let mut count = 0;

    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("count: {}", count);

    /* Para recorrer un array mediante un loop hacemos uso de "for in" */
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    /* El rango (Range) en Rust es proveido por la librería estandar, el cual genera todos los numeros en una secuencía empezando en uno y terminando antes de otro numero */
    for number in 1..4 {
        println!("{}!", number); // 1! 2! 3!
    }

    /* Tambien podemos hacer uso de ".rev()" junto con el rango */
    for number in (1..4).rev() {
        println!("{}!", number); // 3! 2! 1!
    }
}
