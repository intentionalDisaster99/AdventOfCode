use std::collections::HashMap;

fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let mut data: Vec<&str> = get_words(&raw_data);

    // We need to make a new vector that is split along the | in the middle and parsed, that stops at the blank space
    // To make this faster, I think I will try to make it with the whole thing, but only include the lines that have the | in them
    // That way I have less iterations
    let mut rules_vec = data.clone();
    rules_vec.retain(|x| x.contains('|'));

    let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();

    // Populating the rules
    for line in rules_vec {
        // Splitting it into the key and rule
        let parts: Vec<&str> = line.split('|').map(str::trim).collect();

        // Parsing each side
        let key = parts[1].parse::<i64>();
        let value = parts[0].parse::<i64>();

        // Checking for errors, then adding.
        match (key, value) {
            (Ok(k), Ok(v)) => {
                rules.entry(k).or_insert_with(Vec::new).push(v);
            }
            _ => println!("Failed to parse line: {}", line),
        }
    }

    // So now, for each update, we need to retain only those that follow the rules
    // Rule syntax: Look at rules, if matches a key, each value must come before it if it occurs
    data.retain(|row| !row.contains('|') && !row.is_empty()); // Using the data variable to save space

    // Retaining the ones that work
    data.retain(|row| {
        let elements: Vec<i64> = row
            .split(',')
            .filter_map(|num| num.parse::<i64>().ok())
            .collect();

        for (i, element) in elements.iter().enumerate() {
            // If we don't have a key for this one, then we don't need to do stuff
            if !rules.contains_key(element) {
                continue;
            }

            if let Some(to_check) = rules.get(element) {
                for rule in to_check {
                    // If the row contains the rule, it must be at an index less than i
                    if let Some(rule_pos) = elements.iter().position(|&thing| thing == *rule) {
                        // If the rule occurs after the current element, it's invalid
                        if i <= rule_pos {
                            return false;
                        }
                    }
                }
            }
        }

        // If we got this far, we're done
        true
    });

    // Now we just get the middle value and we're gold
    let mut sum = 0;

    for update in data.iter() {
        let elements: Vec<i64> = update
            .split(',')
            .filter_map(|num| num.parse::<i64>().ok())
            .collect();
        sum += elements[(elements.len() - 1) / 2];
    }

    println!("Your answer is {}", sum);
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Five\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}
