use std::collections::HashMap;

use crate::{
    data::DataSet,
    profile::ProfileData,
    recognition::{RecognitionResult, RecognitionSystem, System},
};

pub type ByteBigramRecognition = System<ByteBigramProfileData>;

impl RecognitionSystem for ByteBigramRecognition {
    fn train(&mut self, data: &DataSet) {
        // First we need to process all the data
        for text in &data.data {
            self.classify(text);
        }

        // Once we have iterated over all the texts we need to apply the avg to each profile to finish the vector
        for profile in self.author_profiles.iter_mut() {
            profile.1.data.apply_avg();
        }
    }

    fn test_recognition(&self, test: &DataSet) -> HashMap<String, f64> {
        let mut accuracies: HashMap<String, (f64, f64)> = self
            .author_profiles
            .keys()
            .map(|author| (author.to_owned(), (0.0, 0.0)))
            .collect();
        for test_text in &test.data {
            let result = self
                .recognize(&test_text.text)
                .get_min_distance()
                .unwrap()
                .0;
            let data = accuracies.get_mut(&test_text.author).unwrap();
            data.1 += 1.0; // Add the times that it appeared
            if result == test_text.author {
                data.0 += 1.0; // If we get it right add it
            }
        }
        println!("Accuracies: {:?}", &accuracies);
        accuracies
            .into_iter()
            .map(|entry| (entry.0.to_owned(), entry.1 .0 / entry.1 .1))
            .collect()
    }

    fn recognize(&self, text: &str) -> RecognitionResult {
        let mut prof = ByteBigramProfileData::default();
        prof.process(text);
        prof.apply_avg();
        let mut result = RecognitionResult::default();
        for author_profile in &self.author_profiles {
            let distance = author_profile
                .1
                .data
                .check_difference(&prof, jaccard_distance);
            result.data.push((author_profile.0.to_owned(), distance))
        }
        result
    }
}

#[derive(Debug, Default)]
pub struct ByteBigramProfileData {
    pub profile: HashMap<[u8; 2], f64>,
    pub num_bigrams: f64,
}

impl ProfileData for ByteBigramProfileData {
    fn process(&mut self, txt: &str) {
        let bytes = txt.as_bytes();
        for i in 0..(bytes.len() - 2) {
            let bigram = [bytes[i], bytes[i + 1]];
            let count = self.profile.entry(bigram).or_insert(0.0);
            *count += 1.0;
            self.num_bigrams += 1.0;
        }
    }

    fn check_difference(
        &self,
        other: &Self,
        evaluation_function: fn(own: &Self, other: &Self) -> f64,
    ) -> f64 {
        evaluation_function(self, other)
    }
}

impl ByteBigramProfileData {
    fn apply_avg(&mut self) {
        for entry in self.profile.iter_mut() {
            *entry.1 /= self.num_bigrams;
        }
    }
}

#[allow(dead_code)]
fn jaccard_distance(s: &ByteBigramProfileData, other: &ByteBigramProfileData) -> f64 {
    let set1: Vec<&[u8; 2]> = s.profile.keys().collect();
    let set2: Vec<&[u8; 2]> = other.profile.keys().collect();
    let mut intersection_size = 0;
    for bigram in &set1 {
        if set2.contains(bigram) {
            intersection_size += 1;
        }
    }
    let union_size = set1.len() + set2.len() - intersection_size;
    intersection_size as f64 / union_size as f64
}
