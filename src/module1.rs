pub struct TrainingSet {
    list_of_words: Vec<String>
}

pub fn parse_training_set(path: &str) -> TrainingSet {
    let mut my_list_of_words = Vec::with_capacity(100);

    /*
    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file");
    */


    let training_set_1 = TrainingSet {
        list_of_words: my_list_of_words
    };

    training_set_1
}
