fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // I am just going to loop through until I find mul(
    // then skip if I can't parse the next 3, 2, or 1 digit

    // The total count
    let mut count = 0;

    // Whether it is on or not
    let mut active: bool = true;

    let mut do_count = 0;
    let mut dont_count = 0;

    // Looping for each line
    for line in data.iter() {
        // We just need to split it at the comma and the parenthesis. Then we can do things
        let working_line: Vec<&str> = line.split(|c| c == ')' || c == '(').collect();

        // Now we just loop through until we can find two adjacent elements that we can parse
        for (i, element) in working_line.iter().enumerate() {
            // Checking to see if it ends with do and then if the next two are open bu we only can if the element is more than 1 characer long
            if element.len() > 4 {
                if element[element.len() - 5..] == *"don't" && *working_line[i + 1] == *"" {
                    active = false;
                    println!(
                        "Changed active to {} because of {} and ({})",
                        active,
                        &element,
                        working_line[i + 1]
                    );
                    do_count += 1;
                    continue;
                }
            }
            if element.len() > 1 {
                if element[element.len() - 2..] == *"do" && *working_line[i + 1] == *"" {
                    active = true;
                    println!(
                        "Changed active to {} because of {} and ({})",
                        active,
                        &element,
                        working_line[i + 1]
                    );
                    dont_count += 1;
                    continue;
                }
            }

            // Checking if we can parse it
            if element.split(|c| c == ',').collect::<Vec<_>>().len() != 2 {
                continue;
            }

            // Checking to see if we can parse it
            let num1 = match element.split(|c| c == ',').collect::<Vec<_>>()[0].parse::<i64>() {
                Ok(number) => number,
                Err(_error) => -1,
            };

            // Checking to see if we can parse the second one
            let num2 = match element.split(|c| c == ',').collect::<Vec<_>>()[1].parse::<i64>() {
                Ok(number) => number,
                Err(_error) => -1,
            };

            // If it is negative one then it didn't work
            if num1 == -1 || num2 == -1 {
                continue;
            }

            // Checking to make sure that there was a mul in the element before
            if working_line[i - 1][working_line[i - 1].len() - 3..] != *"mul" {
                continue;
            }

            // If we make it this far, we can add them (assuming we are active)
            if active {
                count += num1 * num2;
            }
        }
    }

    println!("Your answer is {}", count);
    println!("There were {} don'ts and {} dos", dont_count, do_count);
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Three\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}