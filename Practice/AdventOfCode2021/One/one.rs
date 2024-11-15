// Sonar Swap

// Reading from the input file
// use std::env;
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\Practice\\2021\\One\\input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn main() {
    // Getting the data ans splitting it into a vector
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    println!("{}", data.len());

    // TODO Make a new vector to hold the sums of the sliding windows
    let mut sums: Vec<u32> = Vec::new();

    // A number to check against (defaults to the maximum value possible)
    let mut last: u32 = u32::MAX;

    // The total value, which is our output
    let mut sum = 1;

    // This will be the sum of the working amount
    let mut this_sum: u32 = 0;

    // Looping for each value to see if it gets bigger or not
    for (i, &item) in data.iter().enumerate() {
        // The current value
        if let Some(number) = sums.get(i + 1) {
            // Adding to the temp variable
            let temp: u32 = number + &item.trim().parse::<u32>().expect("This gave an error :O");

            // Removing then adding
            sums.remove(i + 1);
            sums.insert(i + 1, temp)
        }

        // The value 1 ahead
        if !(i + 1 > data.len()) {
            // Unwrapping the thing from our thing
            if let Some(number) = sums.get(i + 1) {
                // Adding to the temp variable
                let temp: u32 =
                    number + &item.trim().parse::<u32>().expect("This gave an error :O");

                // Removing then adding
                sums.remove(i + 1);
                sums.insert(i + 1, temp)
            }
        }

        // The value 2 ahead
        if !(i + 2 > data.len()) {
            // Unwrapping the thing from our thing
            if let Some(number) = sums.get(i + 2) {
                // Adding to the temp variable
                let temp: u32 =
                    number + &item.trim().parse::<u32>().expect("This gave an error :O");

                // Removing then adding
                sums.remove(i + 2);
                sums.insert(i + 2, temp)
            }
        }
    }

    // Looping to check to see how many times it increases
    for (_, &item) in sums.iter().enumerate() {
        if item {}
    }

    println!("{}", sum);
    // println!("{}", data[0]);
}

fn get_words(s: &String) -> Vec<&str> {
    // The output vector of all the things
    let mut output: Vec<&str> = Vec::new();

    // Converting the input to bytes
    let bytes = s.as_bytes();

    // The last index so that we can split it again there
    let mut last: usize = 0;

    // Looping through the bytes until we get to the point where we have a new line
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            // Pushing the values into our vector
            output.push(&s[last..i]);

            // Updating the last
            last = i;
        }
    }

    output
}
