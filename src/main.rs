const DICTIONARY: &'static str = include_str!("dictionary_original.txt");

fn main() {
    let lines = DICTIONARY.lines();
    let mut words: Vec<&str> = Vec::new();

    'lines: for line in lines {
        let bad_characters = vec!["(", ")", "[", "]", ".", "*", ",", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

        if line != "" {
            for char in bad_characters {
                if line.contains(char) || line == char {
                    continue 'lines;
                };
            };

            if line.split(' ').collect::<Vec<&str>>().len() == 1 {
                if line.to_uppercase() == line {
                    if !words.contains(&line) {
                        words.push(line);
                        println!("{}", line);
                    };
                };
            };
        };
    };
}
