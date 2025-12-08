mod file_reader;
use std::ops::Add;

use crate::file_reader::{ get_lines, read_contents, read_csv };

fn main() {

    print!("First half answer: {}\n", first_half(false));
    print!("Second half answer: {}\n", second_half(false));

}

fn first_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

    let operations: Vec<String> = data[data.len()-1].split_ascii_whitespace().map(String::from).collect();
    let numbers: Vec<Vec<String>> = data.iter().rev().skip(1).map(|line| line.split_ascii_whitespace().map(String::from).collect()).collect(); 

    let mut out: u128 = 0;

    for (i, operation) in operations.iter().enumerate() {

        let mut current: u128 = numbers[0][i].parse().expect("Bad unwrap of number");
        
        // Doing it for each row 
        for row in numbers.iter().skip(1) {
            if operation == "*" {
                current *= row[i].parse::<u128>().expect("Bad unwrap of number");
            } else {
                current += row[i].parse::<u128>().expect("Bad unwrap of number");
            }
        }

        out += current;

    }

    out

}

fn second_half(test: bool) -> u128 {
    
    let raw_data: String = read_contents(test);
    let data: Vec<Vec<char>> = get_lines(&raw_data).iter().map(|line| line.chars().collect()).collect(); // Indexed as [y][x]
    let mut out: u128 = 0;

    let mut current_operation: char = '.';
    let mut current_numbers: Vec<u128> = vec![];

    // We will be iterating down each column to add numbers to a current buffer, then add/multiply those numbers when we find the empty column

    // Iterating from left to right
    for x in (0..data[0].len()).rev() {

        let mut num_str = "".to_string();

        // Iterating down the column
        for y in 0..data.len() {

            match data[y][x] {
                '*' => { current_operation = '*'; },
                '+' => { current_operation = '+'; },
                _ => { num_str = format!("{}{}", num_str, data[y][x]); }
            };

        }

        // If it is just "" when we trim it, we need to add the numbers
        num_str = num_str.trim().to_string();
        if num_str.len() == 0 {

            out += match current_operation {
                '*' => { current_numbers.iter().fold(1, |prod, x| prod * x) },
                '+' => { current_numbers.iter().fold(0, |sum, x| sum + x) },
                '.' => { panic!("Tried to add to out before finding an operation"); },
                _   => { panic!("How the hell did the operation become {}", current_operation); }
            };

            current_numbers = vec![];

        } else {
            current_numbers.push(num_str.parse::<u128>().expect(&format!("Bad parsing of '{}'", num_str)));
        }



    }

    // Adding the last column in 
    out += match current_operation {
        '*' => { current_numbers.iter().fold(1, |prod, x| prod * x) },
        '+' => { current_numbers.iter().fold(0, |sum, x| sum + x) },
        '.' => { panic!("Tried to add to out before finding an operation"); },
        _   => { panic!("How the hell did the operation become {}", current_operation); }
    };

    out
}

// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_check() {
        let answer: Option<u128> = Some(4277556);
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, first_half(true)),
        }
    }
    
    #[test]
    fn second_half_check() {
        let answer: Option<u128> = Some(3263827);
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, second_half(true)),
        }
    }


}


// Codewars stuff

fn increment_string(s: &str) -> String {
    if s.len() == 0 { return "1".to_string(); }
    let mut characters = s.chars().collect::<Vec<char>>();
    for (i, c) in characters.clone().iter().enumerate().rev() {
        println!("Working with {:#?}", characters);
        if !c.is_ascii_digit() {
            println!("Adding a 1");
            characters.insert(i + 1, '1');
            break;
        } else if c.is_ascii_digit() {
            if c == &'9' {
                characters[i] = '0';
            } else {
                characters[i] = (characters[i].to_string().parse::<u8>().unwrap() + 1).to_string().chars().collect::<Vec<char>>()[0];
                break;
            }
        }
    }
    println!("Ended with {:#?}", characters);
    characters.into_iter().collect::<String>()
}