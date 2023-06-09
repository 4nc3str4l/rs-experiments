use std::collections::HashMap;

use crate::{
    data::DataSet,
    profile::ProfileData,
    recognition::{RecognitionResult, RecognitionSystem, System},
};

const TRACKING_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ ,.;'\"-";

pub type SingleCharacterRecogntion = System<SingleCharProfileData>;

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
        let mut prof = SingleCharProfileData::default();
        prof.process(text);
        prof.apply_avg();

        let mut result = RecognitionResult::default();

        for author_profile in &self.author_profiles {
            let distance = author_profile
                .1
                .data
                .check_difference(&prof, check_difference_v3);
            result.data.push((author_profile.0.to_owned(), distance))
        }
        result
    }
}

#[derive(Debug)]
pub struct SingleCharProfileData {
    pub profile: HashMap<char, f64>,
    pub num_characters: f64,
}

impl SingleCharProfileData {
    fn apply_avg(&mut self) {
        for entry in self.profile.iter_mut() {
            *entry.1 /= self.num_characters;
        }
    }
}

impl Default for SingleCharProfileData {
    fn default() -> Self {
        Self {
            profile: TRACKING_CHARS.chars().map(|c| (c, 0.)).collect(),
            num_characters: 0.0,
        }
    }
}

impl ProfileData for SingleCharProfileData {
    fn process(&mut self, txt: &str) {
        for c in txt.chars() {
            if self.profile.contains_key(&c) {
                let freq = self.profile.get_mut(&c).unwrap();
                *freq += 1.0;
                self.num_characters += 1.;
            }
        }
    }

    fn check_difference(
        &self,
        other: &SingleCharProfileData,
        evaluation_function: fn(s: &SingleCharProfileData, other: &SingleCharProfileData) -> f64,
    ) -> f64 {
        evaluation_function(self, other)
    }
}

#[allow(dead_code)]
fn check_difference(s: &SingleCharProfileData, other: &SingleCharProfileData) -> f64 {
    let mut distance = 0.0;
    for (x0, x1) in s.profile.iter().zip(other.profile.iter()) {
        distance += (*x0.1 - *x1.1).abs();
    }
    distance
}

#[allow(dead_code)]
fn check_difference_v2(s: &SingleCharProfileData, other: &SingleCharProfileData) -> f64 {
    let mut distance = 0.0;
    for (x0, x1) in s.profile.iter().zip(other.profile.iter()) {
        if *x0.1 != 0.0 && *x1.1 != 0.0 {
            distance += (*x0.1 - *x1.1).abs();
        }
    }
    distance
}

#[allow(dead_code)]
fn check_difference_v3(s: &SingleCharProfileData, other: &SingleCharProfileData) -> f64 {
    let mut distance = 0.0;
    // I know that this is terrible as I am doing this each time,
    // it is just to be able to test ideas fast, also I use a bunch of
    // unwraps and so on, please gods of Rust, forgive me for my sins.
    let mut vec: Vec<_> = (s.profile).iter().collect();
    vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let top = vec.into_iter().take(15).collect::<HashMap<_, _>>();

    for x0 in other.profile.iter() {
        if top.contains_key(x0.0) {
            distance += (*x0.1 - *top.get(x0.0).unwrap()).abs();
        }
    }
    distance
}
