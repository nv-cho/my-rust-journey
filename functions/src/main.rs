/* Las funciones siguen la convención snake case. A Rust no le importa el orden de donde han sido declaradas las funciones. */
fn another_function(x: i32, y: i32) {
    println!("Another function.");

    println!("The sum of the values are: {}", x + y);
}

fn main() {
    println!("Hello, world!");

    another_function(2, 3);

    print_labeled_measurements(2, 'm');
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The labeled value is {value}{unit_label}");
}

/*
* Los estatutos (statements) son instrucciones que realizan alguna acción y no devuelven un valor.
* Las expresiones (expressions) evalúan hacía un valor resultante.

* Rust es un lenguaje basado en expresiones (expression-based language), y el cuerpo de las funciones en Rust esta compuesto de statements que opcionalmente terminan en una expresión.
*/
fn statement() {
    let y = 6;

    /* Como los statements no retornan valores, intentar realizar la siguiente acción resultaría en un error */
    let x = let y = 6;
}

/* Las llamadas a funciones, macros, crear un nuevo scope con llaves son todos ejemplos de expresiones */
fn block() -> i32 {

    {
        let x = 5;
    
        /* Las expresiones no incluyen punto y coma, añadir ";" al final de una expresión lo transforma en un statement y no devuelve ningun valor */
        x + 5 
    } 
} 

fn sum(x: i32) -> i32 {
    x + 10
}

fn print_value() {
    let y = sum(2);
    
    println!("The value of y is: {y}");
} 