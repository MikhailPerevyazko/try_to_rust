mod calculating_deposit;
mod cvs_reader;

fn main() {
    // calculating_deposit::count_sum(calculating_deposit::record_struct());
    cvs_reader::read_csv().unwrap();
}
