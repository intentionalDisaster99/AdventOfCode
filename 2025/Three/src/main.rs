mod file_reader;
use crate::file_reader::{ get_lines, read_contents, read_csv };
use std::collections::HashMap;

fn main() {

    print!("First half answer: {}\n", first_half(false));
    print!("Second half answer: {}\n", second_half(false));

}

fn first_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

    let mut sum:u128 = 0;

    for bank in data.iter() {
        sum += get_highest_joltage(bank);
    }

    sum

}

fn get_highest_joltage(input: &String) -> u128 {

    let mut highest: u128 = 0;

    // We can do this by just iterating across each of the batteries to try each combination (the banks are too long for this I don't think)
    for (i, first_battery) in input.chars().collect::<Vec<char>>().iter().enumerate() {

        for second_battery in input[i+1..].to_string().chars().collect::<Vec<char>>().iter() {

            let current = format!("{}{}", first_battery, second_battery).parse::<u128>().unwrap();
            if highest < current {
                highest = current;
            }

        }

    }

    highest
}


fn second_half(test: bool) -> u128 {
    
    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);
    let total_number = data.len();

    let mut sum:u128 = 0;

    for (i, bank) in data.iter().enumerate() {
        sum += get_highest_value(bank, 12);
        println!("{}/{}",i, total_number);
    }

    sum

}

// The pure recursion was very slow, so I memoized it
fn get_highest_value(input: &String, digits: usize) -> u128 {
    let chars: Vec<char> = input.chars().collect();
    let mut memo: HashMap<(usize, usize), u128> = HashMap::new();
    get_highest_value_recursive(&chars, digits, &mut memo)
}

fn get_highest_value_recursive(input: &[char], digits: usize, memo: &mut HashMap<(usize, usize), u128>) -> u128 {

    // Checking the cache
    let state_key = (input.len(), digits);
    if let Some(&cached_val) = memo.get(&state_key) {
        return cached_val;
    }

    // If we have 1 digit, then we just return the max
    if digits == 1 {
        let max=input.iter().map(|e|e.to_string().parse::<u128>().unwrap()).max().unwrap();
        memo.insert(state_key, max);
        return max;
    }

    let mut highest = 0;

    // Checking each index and finding the maximum value that we can after it    
    for (i, battery) in (input[0..=(input.len() - digits)]).iter().enumerate() {
        let current = format!("{}{}", battery, get_highest_value_recursive(&input[(i+1)..], digits - 1, memo)).parse::<u128>().unwrap();
        highest = if current > highest { current } else { highest }
    }

    memo.insert(state_key, highest);
    highest

}



// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_check() {
        assert_eq!(357, first_half(true));
    }
    
    #[test]
    fn second_half_check() {
        assert_eq!(3121910778619, second_half(true));
    }

    #[test]
    fn wrong_value_test() {
        assert_eq!(888911112111, get_highest_value(&"818181911112111".to_string(), 12));
    }


}