mod calculating_deposit;

fn main() {
    let info = calculating_deposit::record_struct();
    calculating_deposit::count_sum(info);
}
