struct Car {
    year: i32,
    model: String,
    company: String,
}

enum Species {
    Crab,
    Fish,
}

enum Weapons {
    Teeth,
    Nails,
}

struct SeaCreature {
    species: Species,
    name: String,
    weapon: Weapons,
}

fn main() {
    let car_one = Car {
        year: 2008,
        model: "i8".to_string(),
        company: "BMW".to_string(),
    };
    let car_two = Car {
        year: 2010,
        model: "a4".to_string(),
        company: "Audi".to_string(),
    };

    println!(
        "First car: model is {}, year - {}, company is {}.",
        car_one.model, car_one.year, car_one.company
    );
    println!(
        "Second car: model is {}, year - {}, company is {}.",
        car_two.model, car_two.year, car_two.company
    );

    let ferries = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferries"),
        weapon: Weapons::Nails,
    };
    let dori = SeaCreature {
        species: Species::Fish,
        name: String::from("Dori"),
        weapon: Weapons::Teeth,
    };

    match ferries.species {
        Species::Crab => println!(
            "{} is a crab! His weapon is {}.",
            ferries.name, ferries.weapon
        ),
        Species::Fish => println!("{} is a fish!", ferries.name),
    }

    match dori.species {
        Species::Crab => println!("{} is a crab!", dori.name),
        Species::Fish => println!("{} is a fish!", dori.name),
    }
}
