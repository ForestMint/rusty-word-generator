
mod module1;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main () {

    let my_training_set = module1::parse_training_set("./some_animals_in_french.txt");

}