use crate::data::extract_datasets;

mod data;
mod recognition;

fn main() {
    let (train, test) = extract_datasets("resources/data.csv", 18000);
    println!("{:?}", train.authors);
    println!("{:?}", test.authors);
}
