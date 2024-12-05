use regex::Regex;

fn main() {
    // Getting the data
    let raw_data: String = read_contents();
    let data: Vec<&str> = get_words(&raw_data);

    // The formats that we are looking for
    let re_front = Regex::new(r"XMAS").unwrap();
    let re_back = Regex::new(r"SAMX").unwrap();

    // The ones that match in the thing
    let mut answer = re_front.captures_iter(&raw_data).collect::<Vec<_>>().len();
    answer += re_back.captures_iter(&raw_data).collect::<Vec<_>>().len();

    // Transposing the matrix
    let mut transposed = vec![String::new(); data[0].len()];

    for element in &data {
        for (i, character) in element.chars().enumerate() {
            transposed[i].push(character);
        }
    }

    // The ones that match in the new thing
    for row in transposed.iter() {
        answer += re_front.captures_iter(&row).collect::<Vec<_>>().len();
        answer += re_back.captures_iter(&row).collect::<Vec<_>>().len();
    }

    // Darn it I just saw that you need diagonals...

    // I think I am going to find all instances of x and try to get the diagonals from there
    for (row_num, row) in data.iter().enumerate() {
        // Looping for each character in the thing
        for (i, character) in row.chars().enumerate() {
            // Checking to see if it is x
            if character != 'X' {
                continue;
            }

            // Now that we know it is an x, we can check to see if the row above or below has the next character
            if row_num > 2 {
                // Checking above left
                if i >= 3 {
                    if data[row_num - 1].chars().nth(i - 1) == Some('M')
                        && data[row_num - 2].chars().nth(i - 2) == Some('A')
                        && data[row_num - 3].chars().nth(i - 3) == Some('S')
                    {
                        answer += 1;
                    }
                }
                // Checking above right

                if data[row_num - 1].chars().nth(i + 1) == Some('M')
                    && data[row_num - 2].chars().nth(i + 2) == Some('A')
                    && data[row_num - 3].chars().nth(i + 3) == Some('S')
                {
                    answer += 1
                }
            }
            // Below
            if row_num < data.len() - 3 {
                // Checking below left
                if i >= 3 {
                    if data[row_num + 1].chars().nth(i - 1) == Some('M')
                        && data[row_num + 2].chars().nth(i - 2) == Some('A')
                        && data[row_num + 3].chars().nth(i - 3) == Some('S')
                    {
                        answer += 1;
                    }
                }
                // Checking below right

                if data[row_num + 1].chars().nth(i + 1) == Some('M')
                    && data[row_num + 2].chars().nth(i + 2) == Some('A')
                    && data[row_num + 3].chars().nth(i + 3) == Some('S')
                {
                    answer += 1;
                }
            }
        }
    }

    println!("The answer is {}", answer);
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:
fn read_contents() -> String {
    let file_path: &str =
        "C:\\Users\\SamWh\\Documents\\Programming\\Rust\\AdventOfCode\\2024\\Four\\Input.txt";

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_words(s: &String) -> Vec<&str> {
    s.lines().collect()
}
