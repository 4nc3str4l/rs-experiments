use crate::{data::{extract_datasets, DataSet}, single_char_recognition::SingleCharacterRecogntion, recognition::RecognitionSystem};

mod data;
mod recognition;
mod single_char_recognition;

fn main() {
    let (train, test) = extract_datasets("resources/data.csv", 18000);
    println!("{:?}", train.authors);
    println!("{:?}", test.authors);
    test_single_char_system(&train, &test);
}

fn test_single_char_system(train: &DataSet, test: &DataSet) {
    let mut system = SingleCharacterRecogntion::default();
    system.train(&train);
    let results = system.test_recognition(&test);
    println!("{:?}", results);
    // Result {"MWS": 0.30327868852459017, "HPL": 0.30176211453744495, "EAP": 0.3469387755102041}

    println!("This text is from MWS");
    println!("{:?}", system.recognize("There, Margaret, the sun is forever visible, its broad disk just skirting the horizon and diffusing a perpetual splendour."));
}
