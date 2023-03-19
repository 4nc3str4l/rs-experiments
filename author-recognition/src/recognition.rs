use std::collections::HashMap;

use crate::data::{DataSet, Text};

struct RecognitionResult {
    data: Vec<(String, f64)>
}

impl RecognitionResult {
    fn get_max(&self) -> Option<(String, f64)> {
        let mut max = std::f64::MIN;
        let mut idx_to_copy = 0;
        for t in self.data.iter().enumerate() {
            if t.1.1 > max {
                idx_to_copy = t.0;
                max = t.1.1;
            }
        }
        self.data.get(idx_to_copy).cloned()
    }
}

trait RecognitionSystem {
    fn train(data: DataSet);
    // Returns accuracy for each author
    fn test_recognition(data: DataSet) -> HashMap<String, f64>;
    // Returns all authors and how
    fn recognize(text: Text) -> RecognitionResult;
}