///Структура, в которой хранятся введенные данные с клавиатуры.
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

pub fn record_struct() -> StorageCountData {
    let sum: f32 = 385000.00;
    let count_info = StorageCountData {
        started_sum: 385000.00,
        percent: 14.2,
        mounths: 3,
        added_sum: 50000.00,
        capitalization: 0.00,
        mounth: 1,
        full_period_percent: 0.00,
        end_sum: sum,
    };
    return count_info;
}

//  Функция, которая рассчитывает вклад.
pub fn count_sum(mut data: StorageCountData) {
    while data.mounth <= data.mounths {
        data.capitalization = (data.started_sum * (data.percent / 100.00)) / 12.00;
        data.end_sum = data.started_sum + data.capitalization;
        println!("% за {} месяц: {} руб.", data.mounth, data.capitalization);
        data.full_period_percent = data.full_period_percent + data.capitalization;
        data.capitalization = 0.0;
        data.started_sum = data.end_sum + data.added_sum;
        data.mounth = data.mounth + 1;
        println!("В конце {} месяца: {} руб.", data.mounth, data.started_sum);
        println!("Общий % = {} руб. \n", data.full_period_percent);
    }
}
