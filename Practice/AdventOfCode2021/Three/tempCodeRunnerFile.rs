fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // An array to count the bits
    let mut one_counts: [usize; 12] = [0; 12];

    // Looping through the data to check each one
    for &item in data.iter() {
        // Looping for each item
        for (i, c) in item.trim().chars().enumerate() {
            if c == '1' {
                one_counts[i] += 1;
            }
        }
    }

    println!("One Counts: {:?}", one_counts);

    // I am going to just add in a a vector for both the oxygen and the co2
    // All of them will start off with 1's in every position, which will be turned to zero when it finds that it won't work
    // Then, at the very end of the vector, I'm going to have a number that represents the number of items left
    // I will decrement it every time I get rid of one

    // Making the vectors
    let mut ox_vec: Vec<u32> = vec![1; data.len()];
    ox_vec.push(data.len() as u32);
    let mut co2_vec: Vec<u32> = vec![1; data.len()];
    co2_vec.push(data.len() as u32);

    // Filtering out the thing
    filter(&data, &mut ox_vec, one_counts, 0, false);
    filter(&data, &mut co2_vec, one_counts, 0, true);

    // Finding the indices of the answers
    let ox_index = ox_vec.iter().position(|&x| x == 1).unwrap_or(0);
    let co2_index = co2_vec.iter().position(|&x| x == 1).unwrap_or(0);

    // The values we want
    let mut ox: u32 = u32::from_str_radix(data[ox_vec[ox_index as usize] as usize], 2).unwrap();
    let mut co2: u32 = u32::from_str_radix(data[co2_vec[co2_index as usize] as usize], 2).unwrap();

    println!(
        "Carbon Dioxide: {}\nOxygen: {}\nAnswer: {}",
        co2,
        ox,
        co2 * ox
    );
}

// My function to filter out the things
// It's recursive :D
// I'm also playing with pass by reference
fn filter(
    data: &Vec<&str>,
    to_filter: &mut Vec<u32>,
    bit_criteria: [usize; 12],
    bit_pos: usize,
    flip: bool,
) {
    // Breaking out if the final value is 1
    if to_filter[to_filter.len() - 1] == 1 {
        println!("Found a value.");
        return;
    }

    // Breaking if the bit_pos gets lower than 0
    if bit_pos >= 12 {
        println!("Too many things matched the criteria");
        return;
    }

    // Figuring out whether or not we want a '0' or '1'
    let mut bit_we_want = '0';
    if bit_criteria[bit_pos] > data.len() / 2 {
        if !flip {
            bit_we_want = '1';
        }
    } else {
        if flip {
            bit_we_want = '1';
        }
    }

    // Filtering out if the nth bit doesn't match
    for i in (0..to_filter.len() - 1).rev() {
        let idx = to_filter[i] as usize;
        if data[idx].chars().nth(bit_pos).unwrap() != bit_we_want {
            to_filter.remove(i);
            let final_index = to_filter.len() - 1;
            to_filter[final_index] -= 1;
        }
    }

    // Calling this function for one bit lower
    filter(data, to_filter, bit_criteria, bit_pos + 1, flip);
}

use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\Three\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
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

    // Adding on the last value
    output.push(&s[last..]);

    output
}
