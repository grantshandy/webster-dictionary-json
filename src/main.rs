use std::{collections::HashMap, str::Lines};
// use serde_json::Value;

const DICTIONARY: &'static str = include_str!("dictionary_original.txt");

fn main() {
    let lines = DICTIONARY.lines();
    collect(lines);
    // let definition = get_definition(480308, lines);
    // println!("\n\ndefinition: {}", definition);
}

fn get_definition(line: usize, lines: Lines) -> String {
    let lines_orig = lines.clone().skip(line + 1);
    let mut current_line: usize = line;

    // Iterate through the lines after our requested line
    for x in lines_orig.clone() {

        // Debugging shit
        current_line += 1;
        println!("{}: {}", current_line, x);

        // Split each line by ':' to check for "Defn"
        let mut split = x.split(':');

        // If there's a Defn before the :
        if  split.nth(0).unwrap() == "Defn" {
            println!("found a defn.");
            // Remove "Defn: " from the line
            let mut x = (&x[6..x.len()]).to_string();

            // If the line has a period in it
            if x.contains('.') {
                // Get the part before the period
                x = x.split('.').nth(0).unwrap().to_string();

                // Then add a period back to it
                x.push('.');

                println!("has a period in it");

                // Return our string out of the function
                return x;
            } else {
                println!("doesn't have a period");

                // If it has no period iterate through the next lines.
                for y in lines.clone().skip(current_line + 1) {
                    if is_word(&y) {
                        panic!("hit a word... {}", y);
                    };

                    println!("        {}", y);

                    // Get the part before the period
                    let y  = y.split('.').nth(0).unwrap().to_string();

                    x.push_str(&format!(" {}.", y));

                    return x;
                };
            };
        };

        if is_word(x) {
            panic!("hit a word... {}", x);
        };
    };

    String::from("placeholder definition")
}

fn is_word(line: &str) -> bool { 
    if line != "" {
        let bad_characters = vec!["(", ")", "[", "]", ".", "*", ",", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

        for char in &bad_characters {
            if line.contains(char) || &line == char {
                return false;
            };
        };

        if line.split(' ').collect::<Vec<&str>>().len() == 1 {
            if line.to_uppercase() == line {
                return true;
            };
        };
    };

    return false;
}

fn collect(lines: Lines) {
    let mut definition_list: HashMap<String, String> = HashMap::new();
    let mut current_line: usize = 0;
    let mut current_letter: char = ' ';

    let bad_characters = vec!["(", ")", "[", "]", ".", "*", ",", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

    'lines: for line in lines.clone().into_iter() {
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

                        let definition = get_definition(current_line, lines.clone());

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
