use std::fs;
use std::fs::read_to_string;

pub struct TrainingSet {
    list_of_words: Vec<String>
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn parse_training_set(path: &str) -> TrainingSet {

    let training_set_1 = TrainingSet {
        list_of_words: read_lines(path)
    };

    training_set_1
}
