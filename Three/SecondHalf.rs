fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // I am going to just add in a a vector for both the oxygen and the co2
    // All of them will start off with 1's in every position, which will be turned to zero when it finds that it won't work
    // Then, at the very end of the vector, I'm going to have a number that represents the number of items left
    // I will decrement it every time I get rid of one

    // Making the vectors
    let mut ox_vec: Vec<u32> = (0..data.len() as u32).collect();
    let mut co2_vec: Vec<u32> = ox_vec.clone();

    // The length of each vector
    let mut ox_length: usize = ox_vec.len();
    let mut co2_length: usize = co2_vec.len();

    // Filtering out the thing
    filter(&data, &mut ox_vec, 0, false, &mut ox_length);
    filter(&data, &mut co2_vec, 0, true, &mut co2_length);

    // Finding the indices of the answers
    let ox_index = ox_vec.iter().position(|&x| x == 1).unwrap_or(0);
    let co2_index = co2_vec.iter().position(|&x| x == 1).unwrap_or(0);

    // The values we want
    let ox: u32 = u32::from_str_radix(data[ox_vec[ox_index as usize] as usize], 2).unwrap();
    let co2: u32 = u32::from_str_radix(data[co2_vec[co2_index as usize] as usize], 2).unwrap();

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
    bit_pos: usize,
    flip: bool,
    length: &mut usize,
) {
    // Breaking out if the final value is 1
    if *length == 1 {
        println!("Found a value.");
        return;
    }

    // Breaking if the bit_pos gets lower than 0
    if bit_pos >= 12 {
        println!("Too many things matched the criteria");
        return;
    }

    // Finding the bit criteria
    let count_ones = to_filter
        .iter()
        .filter(|&&idx| data[idx as usize].chars().nth(bit_pos).unwrap() == '1')
        .count();

    // Figuring out whether or not we want a '0' or '1'
    let bit_we_want = if count_ones * 2 >= *length {
        if flip {
            '0'
        } else {
            '1'
        }
    } else {
        if flip {
            '1'
        } else {
            '0'
        }
    };

    // Filter out indices that do not match the desired bit
    to_filter.retain(|&idx| {
        if data[idx as usize].chars().nth(bit_pos).unwrap() == bit_we_want {
            true
        } else {
            *length -= 1;
            false
        }
    });

    // Calling this function for one bit lower
    filter(data, to_filter, bit_pos + 1, flip, length);
}

use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\Three\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}
