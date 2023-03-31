struct City {
    description: String,
    is_coastal: bool,
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            is_coastal: true,
        }
    } else {
        City {
            description: format!(
                "a *non coastal* city of approximately {} residents",
                residents
            ),
            is_coastal: false,
        }
    }
}

fn main() {
    let rustville: City = new_city(1_000_000, true);

    println!("This city can be described as: {}", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
