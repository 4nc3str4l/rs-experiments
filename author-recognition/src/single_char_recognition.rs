use std::collections::HashMap;

use crate::{
    data::{DataSet, Text},
    recognition::{RecognitionResult, RecognitionSystem},
};

const TRACKING_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.;'\"-";

#[derive(Debug)]
pub struct ProfileData {
    pub profile: HashMap<char, f64>,
    pub num_characters: f64,
}

impl Default for ProfileData {
    fn default() -> Self {
        Self {
            profile: TRACKING_CHARS.chars().map(|c| (c, 0.)).collect(),
            num_characters: 0.0,
        }
    }
}

impl ProfileData {
    fn process(&mut self, txt: &str) {
        for c in txt.chars() {
            if self.profile.contains_key(&c) {
                let freq = self.profile.get_mut(&c).unwrap();
                *freq += 1.0;
                self.num_characters += 1.;
            }
        }
    }

    fn apply_avg(&mut self) {
        for entry in self.profile.iter_mut() {
            *entry.1 /= self.num_characters;
        }
    }

    fn check_difference(&self, other: &ProfileData) -> f64 {
        let mut distance = 0.0;
        for (x0, x1) in self.profile.iter().zip(other.profile.iter()) {
            distance += (*x0.1 - *x1.1).abs();
        }
        distance
    }
}

#[derive(Debug)]
pub struct SingleCharAuthorProfile {
    pub author: String,
    pub data: ProfileData,
}

impl SingleCharAuthorProfile {
    pub fn new(author: String) -> Self {
        Self {
            author,
            data: ProfileData::default(),
        }
    }
}

#[derive(Default)]
pub struct SingleCharacterRecogntion {
    pub author_profiles: HashMap<String, SingleCharAuthorProfile>,
}

impl SingleCharacterRecogntion {
    fn classify(&mut self, txt: &Text) {
        let author = &txt.author;
        if !self.author_profiles.contains_key(&txt.author) {
            self.author_profiles.insert(
                author.to_owned(),
                SingleCharAuthorProfile::new(author.to_owned()),
            );
        }
        let target_profile = self.author_profiles.get_mut(author).unwrap();
        target_profile.data.process(&txt.text);
    }
}

impl RecognitionSystem for SingleCharacterRecogntion {
    fn train(&mut self, data: &DataSet) {
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
        let mut prof = ProfileData::default();
        prof.process(text);
        prof.apply_avg();

        let mut result = RecognitionResult::default();

        for author_profile in &self.author_profiles {
            let distance = author_profile.1.data.check_difference(&prof);
            result.data.push((author_profile.0.to_owned(), distance))
        }
        result
    }
}
