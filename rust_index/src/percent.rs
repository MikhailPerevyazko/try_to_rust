pub fn count_sum(
    mut started_sum: f32,
    mut capitalization: f32,
    mut end_sum: f32,
    percent: f32,
    mounths: i32,
    added_sum: f32,
) -> f32 {
    let mut mounth: i32 = 0;
    let mut full_period_percent: f32 = 0.0;
    while mounth < mounths {
        capitalization = started_sum * (percent / 100.00) / 12.00;
        end_sum = started_sum + capitalization;
        full_period_percent = full_period_percent + capitalization;
        capitalization = 0.0;
        started_sum = end_sum + added_sum;
        mounth += 1;
        println!("В конце {} месяца: {} руб.", mounth, started_sum);
        println!("Общий %: {} руб. \n", full_period_percent);
    }
    return started_sum;
}
