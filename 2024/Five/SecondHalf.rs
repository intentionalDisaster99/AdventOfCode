use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher, RandomState};

fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let mut data: Vec<&str> = get_words(&raw_data);

    // We need to make a new vector that is split along the | in the middle and parsed, that stops at the blank space
    // To make this faster, I think I will try to make it with the whole thing, but only include the lines that have the | in them
    // That way I have less iterations
    let mut rules_vec = data.clone();
    rules_vec.retain(|x| x.contains('|'));

    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();

    // Populating the rules
    for line in rules_vec {
        // Splitting it into the key and rule
        let parts: Vec<&str> = line.split('|').map(str::trim).collect();

        // Parsing each side
        let key = parts[1];
        let value = parts[0];

        // Checking for errors, then adding.
        rules.entry(key).or_insert_with(Vec::new).push(value);
    }

    // So now, for each update, we need to retain only those that follow the rules
    // Rule syntax: Look at rules, if matches a key, each value must come before it if it occurs
    data.retain(|row| !row.contains('|') && !row.is_empty()); // Using the data variable to save space

    // Retaining the ones that don't work
    data.retain(|row| {
        let elements: Vec<_> = row.split(',').collect();

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
                        if elements.contains(rule) && i <= rule_pos {
                            return true;
                        }
                    }
                }
            }
        }

        // If we got this far we don't want it
        false
    });

    // The sum that we are looking for
    let mut sum = 0;

    // Now I just need to order them
    for update in data.iter_mut() {
        // We need to make a string that fits this, so I'll work with a vector then join it
        let mut working_update: Vec<_> = update.split(",").collect();

        // Switching the places of the values and the keys if they don't match until it works
        loop {
            // Looping through each element and checking to see if they work or if we need to switch things
            for (i, element) in working_update.clone().iter().enumerate() {
                // If there is no rule, we don't have to worry about it
                if !rules.contains_key(element) {
                    continue;
                }

                // We now know that there is a rule to follow, so we need to fix it
                // If the key occurs, then each rule must come before it, so we iterate from the back
                // if we hit a rule before we hit the key, we switch them, the break to see if we're done
                for (j, to_check) in working_update.clone().iter().rev().enumerate() {
                    // Done if it is the key
                    if to_check == element {
                        break;
                    }

                    // Switching if we hit a rule
                    if rules.get(element).unwrap().contains(to_check) {
                        let j_index = working_update.len() - 1 - j;
                        working_update.swap(i, j_index);
                        break;
                    }
                }
            }

            // A flat to continue or not
            let mut to_be_or_not_to_be = true;

            // Checking to see if we're done
            for (i, element) in working_update.iter().enumerate() {
                // If we don't have a key for this one, then we don't need to do stuff
                if !rules.contains_key(element) {
                    continue;
                }

                if let Some(to_check) = rules.get(element) {
                    for rule in to_check {
                        // If the row contains the rule, it must be at an index less than i
                        if let Some(rule_pos) =
                            working_update.iter().position(|&thing| thing == *rule)
                        {
                            // If the rule occurs after the current element, it's invalid
                            if working_update.contains(rule) && i <= rule_pos {
                                to_be_or_not_to_be = false;
                            }
                        }
                    }
                }
            }

            // If we got this far, we're done
            if to_be_or_not_to_be {
                break;
            }
        }

        // Adding in the middle value
        sum += working_update[(working_update.len() - 1) / 2]
            .parse::<i64>()
            .unwrap();
    }

    println!("Your answer is {}", sum);
}

// I don't want to deal with crates rn, so this is my random number generator
pub fn rand() -> u64 {
    RandomState::new().build_hasher().finish()
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
