fn main() {
    /* Las variables son inmutables por defecto, añadir la palabra "mut" despues de "let" transforma la variable a una mutable.*/
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    /* Las constantes siempre son inmutables, y su tipo debe ser declarado. Ademas, el valor de las constantes tiene que ser definido a una expresión constante, no puede ser a un valor obtenido en proceso de ejecución */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    /*
    * Shadowing es como se llama reasignar variables declarandolas. Al hacer esto efectivamente estamos redeclarando la variable. Esto nos permite cambiar su tipo conservando sin necesidad de cambiar el nombre  de la variable

    * Cuando hacemos shadowing sobre una variable no estamos afectando su caracteristica de inmutabilidad/mutabilidad. Si hacemos shadowing de una variable inmutable, luego de redeclararla su valor sigue siendo inmutable.
    */
    let i = 10;

    /* Imprime "10" */
    println!("The value of i is: {i}");

    {
        let i = 20;

        /* Imprime "20" */
        println!("The value of i is: {i}");
    }

    let i = 5.0;

    /* Imprime "5.0" */
    println!("The value of i is: {i}");
}
