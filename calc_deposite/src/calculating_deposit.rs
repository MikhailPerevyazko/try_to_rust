use std::io;
use std::io::Write;

//Структура, хранящая данные дял вычислений.
#[derive(Debug)]
pub struct StorageCountData {
    pub started_sum: f32,
    pub percent: f32,
    pub mounths: i32,
    pub added_sum: f32,
    pub capitalization: f32,
    pub mounth: i32,
    pub full_period_percent: f32,
    pub end_sum: f32,
}

// Функция, считывающая значения с клавиатуры.
fn input_data(comment: &str) -> String {
    print!("{}", comment);
    let _ = io::stdout().flush();
    let mut string_value: String = String::new();

    io::stdin()
        .read_line(&mut string_value)
        .ok()
        .expect("Error read line!");

    return string_value;
}

// Функция, создающая структуру и записывающая в нее введенные данные.
pub fn record_struct() -> StorageCountData {
    let input_started_sum: String = input_data("Введите размер вклада: ");
    let input_percent: String = input_data("Введите % вклада: ");
    let input_mounths: String = input_data("Введите срок вклада: ");
    let input_added_sum: String = input_data("Введите ежемесячно пополняемую сумму: ");

    let parsed_input_started_sum: f32 = input_started_sum.trim().parse::<f32>().unwrap();
    let parsed_input_percent: f32 = input_percent.trim().parse::<f32>().unwrap();
    let parsed_input_mounths: i32 = input_mounths.trim().parse::<i32>().unwrap();
    let parsed_input_added_sum: f32 = input_added_sum.trim().parse::<f32>().unwrap();

    let sum: f32 = parsed_input_started_sum;
    let count_info: StorageCountData = StorageCountData {
        started_sum: parsed_input_started_sum,
        percent: parsed_input_percent,
        mounths: parsed_input_mounths,
        added_sum: parsed_input_added_sum,
        capitalization: 0.00,
        mounth: 0,
        full_period_percent: 0.00,
        end_sum: sum,
    };
    return count_info;
}

//  Функция, которая рассчитывает вклад.
pub fn count_sum(mut data: StorageCountData) {
    // Вывод начальных данных.
    println!("\nРазмер вклада: {} руб.\nПроцент вклада: {} %.\nПериод вклада: {} мес. \nСумма ежемесячного пополнения: {} руб.\n", 
    data.started_sum, data.percent, data.mounths, data.added_sum);
    while data.mounth < data.mounths {
        // Расчет прцентов за текуций месяц.
        data.capitalization = (data.started_sum * (data.percent / 100.00)) / 12.00;
        // Расчет конечной суммы.
        data.end_sum = data.started_sum + data.capitalization;
        data.mounth = data.mounth + 1;
        println!("% за {} месяц: {} руб.", data.mounth, data.capitalization);
        // Расчет общего начисления процентов.
        data.full_period_percent = data.full_period_percent + data.capitalization;
        data.capitalization = 0.0;
        // Расчет суммы, на которую будет начислятсья процент в следующем месяце.
        data.started_sum = data.end_sum + data.added_sum;
        println!("В конце {} месяца: {} руб.", data.mounth, data.started_sum);
        println!("Общий % = {} руб. \n", data.full_period_percent);
    }
}
