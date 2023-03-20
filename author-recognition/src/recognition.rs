use std::collections::HashMap;

use crate::data::DataSet;

#[derive(Debug, Default)]
pub struct RecognitionResult {
    pub data: Vec<(String, f64)>,
}

impl RecognitionResult {
    pub fn get_min_distance(&self) -> Option<(String, f64)> {
        let mut min = std::f64::MAX;
        let mut idx_to_copy = 0;
        for t in self.data.iter().enumerate() {
            if t.1 .1 < min {
                idx_to_copy = t.0;
                min = t.1 .1;
            }
        }
        self.data.get(idx_to_copy).cloned()
    }
}

pub trait RecognitionSystem {
    // Trains the system
    fn train(&mut self, data: &DataSet);
    // Returns accuracy for each author
    fn test_recognition(&self, test: &DataSet) -> HashMap<String, f64>;
    // Returns all authors and how
    fn recognize(&self, text: &str) -> RecognitionResult;
}
