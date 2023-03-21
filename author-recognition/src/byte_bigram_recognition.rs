use std::collections::HashMap;

use crate::{recognition::{System, RecognitionSystem, RecognitionResult}, profile::ProfileData, data::DataSet};

type ByteBigramRecognition = System<ByteBigramProfileData>;

impl RecognitionSystem for ByteBigramRecognition {
    fn train(&mut self, data: &DataSet) {
        todo!()
    }

    fn test_recognition(&self, test: &DataSet) -> HashMap<String, f64> {
        todo!()
    }

    fn recognize(&self, text: &str) -> RecognitionResult {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct ByteBigramProfileData {

}

impl ProfileData for ByteBigramProfileData {
    fn process(&mut self, txt: &str) {
        todo!()
    }

    fn check_difference(
        &self,
        other: &Self,
        evaluation_function: fn(own: &Self, other: &Self) -> f64,
    ) -> f64 {
        todo!()
    }
}

