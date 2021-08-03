#[macro_use]
extern crate lazy_static;

// use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::RwLock;
use std::sync::Mutex;

const DICT: &'static str = include_str!("original.txt");

lazy_static! {
    static ref DICT_MAP: Mutex<BTreeMap<String, String>> = Mutex::new(BTreeMap::new());
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
    return true;
}