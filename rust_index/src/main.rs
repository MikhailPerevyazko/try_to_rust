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
        Species::Crab => println!("{} is a crab!", ferries.name),
        Species::Fish => println!("{} is a fish!", ferries.name),
    }

    match dori.species {
        Species::Crab => println!("{} is a crab!", dori.name),
        Species::Fish => println!("{} is a fish!", dori.name),
    }

    match ferries.weapon {
        Weapons::Nails => println!("Nails is a crab's weapon!"),
        Weapons::Teeth => println!("Teeth is a fish's weapon!"),
    }

    match dori.weapon {
        Weapons::Nails => println!("Nails is a crab's weapon!"),
        Weapons::Teeth => println!("Teeth is a fish's weapon!"),
    }

    fn do_something(num: i32) -> Result<i32, String> {
        if num == 32 {
            Ok(num)
        } else {
            Err(String::from("Error"))
        }
    }

    let result = do_something(32).unwrap();
    println!("Found {}", result);

    let mut new_vec_one: Vec<i32> = Vec::new();
    new_vec_one.push(1);
    new_vec_one.push(2);

    let mut new_vec_two = vec![
        String::from("Name"),
        String::from("Surname"),
        String::from("Middlename"),
    ];

    new_vec_two.push(String::from("Age"));

    let mut new_vec_three: Vec<String> = vec![];

    for item in new_vec_two.iter() {
        new_vec_three.push(String::from(item));
    }

    println!("{:#?}", new_vec_three);

    // Управление ресурсами на основе обсласти видмости
    struct Foo {
        x: i32,
    }

    struct Bar {
        bar: Foo,
    }

    fn make_foo() {
        let new_foo_one = Foo { x: 11 };
        let new_foo_two = Foo { x: 22 };

        println!("{}", new_foo_one.x);
        println!("{}", new_foo_two.x);

        let foo = Bar { bar: Foo { x: 33 } };
        println!("{}", foo.bar.x);
    }

    make_foo();

    #[derive(Debug)]
    struct Beep {
        x: i32,
    }
    fn print_beep(b: Beep) {
        println!("{}", b.x);
    }

    let beep = Beep { x: 44 };
    print_beep(beep);

    fn move_beep() -> Beep {
        let a = Beep { x: 55 };
        return a;
    }

    let move_beep = move_beep();
    println!("{:?}", move_beep.x);

    let beep_two = Beep { x: 66 };
    let some_beep = &beep_two;
    println!("{}", some_beep.x);

    let started_sum: f32 = 378000.00;
    let percent: f32 = 14.2;
    let mounths: i32 = 5;
    let added_sum: f32 = 60000.00;

    count_sum(started_sum, percent, mounths, added_sum);
}

fn count_sum(mut started_sum: f32, percent: f32, mounths: i32, added_sum: f32) -> f32 {
    let mut mounth: i32 = 0;
    let mut full_period_percent: f32 = 0.0;
    let mut capitalization: f32 = 0.0;
    let mut end_sum: f32 = started_sum;
    while mounth < mounths {
        capitalization = started_sum * (percent / 100.00) / 12.00;
        end_sum = started_sum + capitalization;
        full_period_percent = full_period_percent + capitalization;
        mounth += 1;
        println!("% за {} месяц: {}", mounth, capitalization);
        capitalization = 0.0;
        started_sum = end_sum + added_sum;
        println!("В конце {} месяца: {} руб.", mounth, started_sum);
        println!("Общий %: {} руб. \n", full_period_percent);
    }
    return started_sum;
}
