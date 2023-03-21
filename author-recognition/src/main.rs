use crate::{
    byte_bigram_recognition::ByteBigramRecognition,
    data::{extract_datasets, DataSet},
    recognition::RecognitionSystem,
    single_char_recognition::SingleCharacterRecogntion,
};

mod byte_bigram_recognition;
mod data;
mod profile;
mod recognition;
mod single_char_recognition;

fn main() {
    let (train, test) = extract_datasets("resources/data.csv", 18000);
    println!("{:?}", train.authors);
    println!("{:?}", test.authors);
    println!("Single Character Recognition Results");
    test_system::<SingleCharacterRecogntion>(&train, &test);

    println!("Bigram Recognition Results:");
    test_system::<ByteBigramRecognition>(&train, &test);
}

fn test_system<T: RecognitionSystem + Default>(train: &DataSet, test: &DataSet) {
    println!("Single char system");
    let mut system = T::default();
    system.train(train);
    let results = system.test_recognition(test);
    println!("{results:?}");
    println!("This text is from MWS");
    println!("{:?}", system.recognize("There, Margaret, the sun is forever visible, its broad disk just skirting the horizon and diffusing a perpetual splendour."));
}
