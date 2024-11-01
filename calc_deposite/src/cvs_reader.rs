use csv::Reader;
use std::{error::Error, fs::File};

pub fn read_csv() -> Result<(), Box<dyn Error>> {
    // Открываем CSV файл
    let file = File::open("/home/Mikhail/projects/try_to_rust/calc_deposite/operation.csv")?;

    // Создаем читатель
    let mut rdr = Reader::from_reader(file);

    // Перебираем записи
    for result in rdr.records() {
        let record = result.unwrap();
        println!("{:?}", record);
    }
    Ok(())
}
