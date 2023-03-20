use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Default, Debug)]
pub struct Text {
    pub id: String,
    pub text: String,
    pub author: String,
}

impl Text {
    pub fn from_line(line: &str) -> Option<Self> {
        let split: Vec<&str> = line.split("\",\"").collect();
        if split.len() != 3 {
            return None;
        }
        let id = split[0].to_owned().replace('\"', "");
        let text = split[1].to_owned().replace('\"', "");
        let author = split[2].to_owned().replace('\"', "");
        Some(Self { id, text, author })
    }
}

#[derive(Default, Debug)]
pub struct DataSet {
    pub data: Vec<Text>,
    pub authors: HashMap<String, usize>,
}

impl DataSet {
    fn insert(&mut self, t: Text) {
        if !self.authors.contains_key(&t.author) {
            self.authors.insert(t.author.to_owned(), 0);
        }
        let entry = self.authors.get_mut(&t.author).unwrap();
        *entry += 1;
        self.data.push(t);
    }
}

pub fn extract_datasets(path: &str, train_size: usize) -> (DataSet, DataSet) {
    let mut train_dataset = DataSet::default();
    let mut test_dataset = DataSet::default();
    if let Ok(lines) = read_lines(path) {
        let mut num_read = 0;
        for line in lines.skip(1).flatten() {
            if let Some(writting) = Text::from_line(&line) {
                if num_read < train_size {
                    train_dataset.insert(writting);
                } else {
                    test_dataset.insert(writting);
                }
                num_read += 1;
            }
        }
    }
    (train_dataset, test_dataset)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
