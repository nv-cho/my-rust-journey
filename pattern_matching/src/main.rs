fn main() {
    enum Color {
        Red,
        Blue,
        Green,
        Struct { red: u8, blue: u8, green: u8 },
        Tuple(u8, u8, u8),
    }

    let current_color = Color::Green;

    match current_color {
        Color::Red => {
            println!("It was Red");
        }
        Color::Blue => {
            println!("It was Blue");
        }
        Color::Green => {
            println!("It was Red");
        }
        Color::Struct { red, green, blue } => {
            println!("{}, {}, {}", red, green, blue);
        }
        Color::Tuple(red, green, blue) => {
            println!("{}, {}, {}", red, green, blue);
        }
    }

    /* Asi se puede utilizar match como una expresion */
    enum Tool {
        Hammer,
        Axe,
    }

    let current_tool = Tool::Hammer;

    let color_str = match current_tool {
        Tool::Hammer => "Hammmmer",
        Tool::Axe => "Axxxxxxe",
    };

    /* A la hora de utilizar "match", no es necesario cubrir cada posible valor del enum, es posible realizar lo siguiente: */
    let another_tool = Tool::Axe;

    /* De esta forma podemos especificar una acción para un posibilidad y un acción para todo el resto de posibilidades */
    match another_tool {
        Tool::Axe => {
            println!("Its an AXE")
        }
        _ => {
            println!("I dont care")
        }
    }
}
