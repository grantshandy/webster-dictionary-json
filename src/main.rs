use std::{collections::HashMap, str::Lines};

const DICTIONARY: &'static str = include_str!("dictionary_original.txt");

fn main() {
    let lines = DICTIONARY.lines();
    collect(lines);
    // let definition = get_definition(8137, lines);
    // println!("\n\ndefinition: {}", definition);
}

fn get_definition(line: usize, lines: Lines) -> String {
    let lines_orig = lines.clone().skip(line + 1);
    let mut current_line: usize = line;

    // iterate through the lines after our requested line
    for x in lines_orig.clone() {

        current_line += 1;
        // println!("{}: {}", current_line, x);

        match check_for_defn(x, lines.clone(), current_line) {
            Some(data) => return data,
            None => (),
        };

        match check_for_num(x, lines.clone(), current_line) {
            Some(data) => return data,
            None => (),
        };

        if is_word(&x) {
            panic!("hit a word before a definition... {}, {}", current_line, x);
        };
    };

    String::from("Couldn't detect a definition")
}

fn check_for_num(x: &str, lines: Lines, current_line: usize) -> Option<String> {
    // Check for number
    if x.split('.').nth(0).unwrap() == "1" {
        // remove "1. " from the line
        let mut x = (&x[3..x.len()]).to_string();

        // check for a period in the line
        if x.contains('.') {
            // check for whatever the dumb thing is
            if x.chars().nth(0).unwrap() == '(' && x.chars().last().unwrap() == ')' {
                for y in lines.clone().skip(current_line + 1) {
                    // println!("{}: {}", current_line, y);
                    
                    match check_for_defn(y, lines.clone(), (current_line + 2)) {
                        Some(data) => {
                            // println!("data: {}", data);
                            return Some(data);
                        },
                        None => (),
                    };
                };
            };

            x = x.split('.').nth(0).unwrap().to_string();
            x = format!("{}.", x);

            return Some(x);
        } else {
            // if it has no period iterate through the next lines.
            for y in lines.clone().skip(current_line + 1) {
                // println!("        {}", y);

                if y.contains('.') {
                    let y  = y.split('.').nth(0).unwrap().to_string();

                    x.push_str(&format!(" {}.", y));

                    return Some(x);
                } else {
                    x.push_str(&format!(" {}", y));
                };
            };
        };
    };
    
    return None;
}

fn check_for_defn(x: &str, lines: Lines, current_line: usize) -> Option<String> {
    // split each line by ':' to check for "Defn"
    let mut split = x.split(':');

    // if there's a Defn before the :
    if  split.nth(0).unwrap() == "Defn" {
        // println!("found a defn...");
        // remove "Defn: " from the line
        let mut x = (&x[6..x.len()]).to_string();

        // if the line has a period in it
        if x.contains('.') {
            // get the part before the period
            x = x.split('.').nth(0).unwrap().to_string();

            // then add a period back to it
            x.push('.');

            // println!("has a period in it");

            // return our string out of the function
            return Some(x);
        } else {
            // println!("doesn't have a period");

            // if it has no period iterate through the next lines.
            for y in lines.clone().skip(current_line + 1) {
                if y.contains('.') {
                    let y  = y.split('.').nth(0).unwrap().to_string();

                    // println!("pushing {}", y);
                    x.push_str(&format!(" {}.", y));

                    return Some(x);
                } else {
                    // println!("pushing {}", y);
                    x.push_str(&format!(" {}", y));
                };
            };
        };
    };

    return None;
}

fn is_word(line: &str) -> bool { 
    if line != "" {
        let bad_characters = vec!["(", ")", "[", "]", ".", "*", ",", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

        for char in bad_characters {
            if line.contains(char) || line == char {
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
                    let first_letter = word.chars().nth(0).unwrap();

                    if first_letter != current_letter {
                        if first_letter == '\'' || first_letter == '-' {
                            continue 'lines;
                        } else {
                            current_letter = first_letter;
                            println!("Processing letter {}...", current_letter);
                        }
                    };

                    let word = word.to_lowercase();

                    let definition = get_definition(current_line, lines.clone());

                    println!("{} - {}", word, definition);

                    // let definition = get_definition(current_line, lines.clone());

                    // definition_list.insert(word.to_string().to_lowercase(), definition);
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
