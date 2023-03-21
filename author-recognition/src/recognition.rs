use std::collections::HashMap;

use crate::{
    data::{DataSet, Text},
    profile::{Profile, ProfileData},
};

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

#[derive(Default)]
pub struct System<T: ProfileData + Default> {
    pub author_profiles: HashMap<String, Profile<T>>,
}

impl<T: ProfileData + Default> System<T> {
    pub fn classify(&mut self, txt: &Text) {
        let author = &txt.author;
        if !self.author_profiles.contains_key(&txt.author) {
            self.author_profiles
                .insert(author.to_owned(), Profile::<T>::new(author));
        }
        let target_profile = self.author_profiles.get_mut(author).unwrap();
        target_profile.data.process(&txt.text);
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
