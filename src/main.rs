use std::collections::HashMap;
// use serde_json::Value;

const DICTIONARY: &'static str = include_str!("dictionary_original.txt");

fn main() {
    let lines = DICTIONARY.lines();
    let mut definition_list: HashMap<String, String> = HashMap::new();
    let mut current_line: usize = 0;
    let mut current_letter: char = 'A';

    let bad_characters = vec!["(", ")", "[", "]", ".", "*", ",", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

    'lines: for line in lines {
        current_line += 1;

        if line != "" {
            for char in &bad_characters {
                if line.contains(char) || &line == char {
                    continue 'lines;
                };
            };

            if line.split(' ').collect::<Vec<&str>>().len() == 1 {
                let word = line;
                if word.to_uppercase() == word {
                    if !definition_list.contains_key(&word.to_string()) {
                        let first_letter = word.chars().nth(0).unwrap();

                        if first_letter != current_letter {
                            if first_letter == '\'' || first_letter == '-' {
                                continue 'lines;
                            } else {
                                current_letter = first_letter;
                                println!("Processing letter {}...", current_letter);
                            }
                        };

                        let definition = get_definition(word, current_line);

                        definition_list.insert(word.to_string().to_lowercase(), definition);
                    };
                };
            };
        };
    };

    let query = "stupid";

    match definition_list.get(query) {
        Some(data) => println!("{} definition: {}", query, data),
        None => eprintln!("no data for {}", query),
    }

}

fn get_definition(_word: &str, _line: usize) -> String {
    String::from("placeholder definition")
}