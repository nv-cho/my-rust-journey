fn main() {
    /* Esta es la forma de declarar una enumeración (enum) de, en este caso, de tres posibles valores */
    enum Color {
        Green,
        Yellow,
        Red,
    }

    /* Asi es como podemos acceder a ellos, las tres variables tienen el mismo tipo, "Color", pero las tres tienen los tres posibles valores de "Color" */
    let go = Color::Green;
    let stop = Color::Red;
    let slow_down = Color::Yellow;

    /*Los enums pueden tener valores que sean estructuras o tuplas */
    enum Struct {
        Green,
        Yellow,
        Red,
        Custom { red: u8, green: u8, blue: u8 },
    }

    let _purple = Struct::Custom {
        red: 100,
        green: 0,
        blue: 250,
    };

    enum Tuple {
        Green,
        Yellow,
        Red,
        Custom(u8, u8, u8),
    }

    let purple = Tuple::Custom(100, 0, 250);
}
