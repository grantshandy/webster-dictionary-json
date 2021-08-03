#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
use std::sync::Mutex;

const DICT: &'static str = include_str!("original.txt");

lazy_static! {
    static ref DICT_MAP: Mutex<BTreeMap<String, String>> = Mutex::new(BTreeMap::new());
    static ref WORDS_DONT_CONTAIN: Vec<&'static str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "(", ")", ":", ";", "[", "]", ".", ",", "|", " "];
}


fn main() {
    let lines = DICT.lines();

    for line in lines {
        if check_line_is_word(line) {
            DICT_MAP.lock().unwrap().insert(line.to_string(), "value".to_string());
        }
    }

    print_pretty_dict();
}

fn print_pretty_dict() {
    for (word, defn) in DICT_MAP.lock().unwrap().clone().into_iter() {
        println!("{}: {}", word, defn);
    }
}

fn check_line_is_word(line: &str) -> bool {
    if line.to_uppercase() == line {
        for bad_char in WORDS_DONT_CONTAIN.clone().into_iter() {
            if !line.contains(bad_char) {
                return true;
            }
        }
    }

    return false;
}