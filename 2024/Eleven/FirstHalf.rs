fn main() {
    // First, we have to read the data
    let raw_data: String = read_contents(false);
    let mut data: Vec<u128> = raw_data
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    // Looping for each blink
    for blink_num in 0..25 {
        blink(&mut data);
        println!("Finished blink {}", blink_num);
    }

    // The length
    println!("There are {} stones", data.len());
}

// What happens when bro blinks
fn blink(data: &mut Vec<u128>) {
    // Iterating through each stone (backwards so we don't mess up the i)
    for (i, number) in data.clone().iter().enumerate().rev() {
        // If it is zero, we change it to a one
        if *number == 0 {
            data[i] = 1;
        } else if format!("{}", number).len() % 2 == 0 {
            // Turning it to a string to split
            let as_string: String = format!("{}", number);

            // Adding each half back in
            data[i] = as_string[..(as_string.len() / 2)].parse::<u128>().unwrap();
            data.insert(
                i + 1,
                as_string[(as_string.len() / 2)..].parse::<u128>().unwrap(),
            );
        } else {
            // Multiplying by 2024
            data[i] *= 2024;
        }
    }
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:

fn read_contents(test: bool) -> String {
    let mut file_path: &str = "Testing.txt";
    if !test {
        file_path = "Input.txt";
    }

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn _get_words(s: &String) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
