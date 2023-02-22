use hw5_6::borrowing_house::run_borrowing_provider;
use hw5_6::owning_house::run_owning_provider;

fn main() {
    println!("Owning provider:");
    run_owning_provider();

    println!("Borrowing provider:");
    run_borrowing_provider();
}
