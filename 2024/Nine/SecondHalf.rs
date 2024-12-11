fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data_lines: Vec<Vec<char>> = get_words(&raw_data);

    // Splitting by characters
    let data: Vec<char> = data_lines[0].clone();

    // Now we want to make a new vector that describes the file blocks with their ID numbers
    let mut file_blocks: Vec<Vec<String>> = collapse_data(&data);

    // Now we need to compress the thing to make it smaller like they want
    squish(&mut file_blocks);

    // Now to count
    for block in file_blocks.iter() {
        for part in block.iter() {
            print!("{}", part);
        }
    }
    println!();
    println!("Your answer is {}", checksum(file_blocks.clone()));
}

// Counting it up
fn checksum(input: Vec<Vec<String>>) -> u64 {
    let mut output: u64 = 0;

    let mut index = 0;

    for block in input.iter() {
        for character in block.iter() {
            if character.contains('.') {
                index += 1;
                continue;
            }

            output += character.to_string().trim().parse::<u64>().unwrap() * (index as u64);
            index += 1;
        }
    }

    output
}

// Moves the data around to make it smaller
fn squish(blocks: &mut Vec<Vec<String>>) {
    // Blocks is in the format
    // [ ["num", "num", "num"], [".", ".", "."], ["num", "num"]]

    // The current element
    let mut element_index: usize = blocks.len() - 1; // Safe subtraction (in case it is zero)

    loop {
        // Making sure that the blocks don't have any consecutive "." because that could through it off
        compress(blocks);

        // println!(
        //     "Index is {} and points to {:?}",
        //     element_index, blocks[element_index]
        // );

        // for block in blocks.iter() {
        //     for part in block.iter() {
        //         print!("{}", part);
        //     }
        // }
        // println!();

        // Updating the element
        let element = blocks[element_index].clone();

        // If it is a ., then we don't play with it yet
        if element[0].contains('.') {
            // println!("Skipping {:?} because it has . in it", element);
            element_index -= 1;
            continue;
        }
        // println!("Processing for {:?}", element);

        // We need to find the left most slot that has that space
        // I'm using an option so that I have a none value
        let mut dots_index: Option<usize> = None;
        // Find a suitable block with enough dots
        for (i, block) in blocks.iter().enumerate() {
            if block[0].contains('.') && block.len() >= element.len() {
                dots_index = Some(i);
                break;
            }
        }

        // Making sure we aren't moving it to the right
        if dots_index.is_none() || element_index < dots_index.unwrap() {
            // Checking to see if we can go to the next one
            if element_index == 0 {
                // println!("Broken out at zero");
                break;
            }
            element_index -= 1;
            // println!(
            //     "Skipping {:?} because we couldn't find a matching one",
            //     element
            // );
            continue;
        }

        if let Some(dots_idx) = dots_index {
            let dots = &mut blocks[dots_idx];
            // println!("Processing dots block at index {}: {:?}", dots_idx, dots);

            if dots.len() == element.len() {
                // println!("Swapping {:?} and {:?}", element, blocks[dots_idx]);
                blocks.swap(element_index, dots_idx);
            } else {
                // Splitting the dot block properly
                let moving: Vec<String> = dots[element.len()..].to_vec();
                let remaining: Vec<String> = dots[..element.len()].to_vec();

                *dots = moving;
                blocks.insert(dots_idx, remaining);
                // println!(
                //     "Swapping {:?} and {:?}",
                //     blocks[element_index + 1],
                //     blocks[dots_idx]
                // );
                blocks.swap(element_index + 1, dots_idx);

                // Increasing the index so that later when we decrement we don't skip
                element_index += 1;
            }
        }

        // We are done if we hit the final element
        if element_index == 0 {
            // println!("Finished squishing");
            break;
        }
        // println!(
        //     "Changing element index to point to {:?} from {:?}",
        //     blocks[element_index - 1],
        //     blocks[element_index]
        // );
        element_index -= 1;
    }
    // println!("Leaving Squish:");
    // for block in blocks.iter() {
    //     for part in block.iter() {
    //         print!("{}", part);
    //     }
    // }
    // println!();
    // One final squish
    compress(blocks);
}

// Compresses down every "." if it can
fn compress(blocks: &mut Vec<Vec<String>>) {
    // println!("Entering Compress:");
    // for block in blocks.iter() {
    //     for part in block.iter() {
    //         print!("{}", part);
    //     }
    // }
    // println!();
    let mut i = 0;
    while i < blocks.len() - 1 {
        // Only merge if both blocks are non-empty and contain '.'
        if blocks[i].len() > 0
            && blocks[i + 1].len() > 0
            && blocks[i][0].contains('.')
            && blocks[i + 1][0].contains('.')
        {
            let to_merge = blocks[i + 1].clone(); // Clone the entire vector to merge
            blocks[i].extend(to_merge); // Merge the vectors
            blocks.remove(i + 1); // Remove the merged block
        } else {
            i += 1;
        }
    }
    // for block in blocks.iter() {
    //     for part in block.iter() {
    //         print!("{}", part);
    //     }
    // }
    // println!();
}

// Collapses data into a representation of the file blocks by their index
fn collapse_data(input: &Vec<char>) -> Vec<Vec<String>> {
    let mut expanded: Vec<Vec<String>> = input
        .iter()
        .enumerate()
        .map(|(index, representation)| {
            // Here, we have to decode the number and add it that many times
            // These will all be their own vectors that we will have to collapse later

            let mut output: Vec<String> = Vec::new();
            for _ in 0..representation.to_string().parse::<usize>().unwrap() {
                if index % 2 == 0 {
                    output.push((index / 2).to_string());
                } else {
                    output.push(".".to_string());
                }
            }

            output
        })
        .collect::<Vec<Vec<String>>>();

    // Making sure that there are no empty places
    expanded = expanded
        .clone()
        .into_iter()
        .filter(|x| x.len() > 0) //|x| x.is_empty() || *x[0] != "".to_string())
        .collect::<Vec<Vec<String>>>();

    expanded
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:

fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Nine\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
