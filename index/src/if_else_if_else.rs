use std::io;

pub fn learning() {
    let x: i32 = 15;
    if x <= 0 {
        println!("X меньше или равно 0");
    } else if x > 0 && x <= 15 {
        println!("Х лежит в пределах от 1 до 15")
    } else {
        println!("Х больше 15")
    }
}

pub fn infinity_loop() {
    let mut x = 0;
    loop {
        x += 1;
        println!("{x}");
        if x == 10 {
            break;
        }
    }
}

pub fn find_loop() {
    let mut x: i32 = 0;
    let find_value = loop {
        x += 1;
        if x == 15 {
            break "15";
        }
    };
    println!("Find value from loop is {}!", find_value);
}

pub fn percent_count() -> f32 {
    let mut wanted_sum: String = String::new();
    let mut percent: String = String::new();
    let mut period: String = String::new();

    println!("Введите желаемую сумму в месяц:");
    io::stdin()
        .read_line(&mut wanted_sum)
        .expect("Вы ввели неравльную сумму!");
    println!("Введите процент на остаток:");
    io::stdin()
        .read_line(&mut percent)
        .expect("Вы ввели неправильынй процент!");
    io::stdin()
        .read_line(&mut period)
        .expect("Вы ввели неправильный период!");

    let cut_wanted_sum: String = wanted_sum
        .trim()
        .parse()
        .expect("Введите корректную сумму!");
    let cut_percent: String = percent.trim().parse().expect("Введите корректный процент!");
    let cut_period: String = period.trim().parse().expect("Введите корректный период!!");

    let parsed_wanted_sum = cut_wanted_sum.parse::<f32>().unwrap();
    let parsed_percent = cut_percent.parse::<f32>().unwrap();
    let parsed_period = cut_period.parse::<f32>().unwrap();

    let sum: f32 = (parsed_wanted_sum * 12.00) / (parsed_percent / 100.00);
    let ceil_sum = sum.ceil();

    println!("Нужная сумма равна: {}.", ceil_sum);

    return sum;
}
