use std::collections::HashMap; // For the memoization

// I HAD NO IDEA IT WAS THAT EASY WHAT
// (I didn't push the other ones, but there are two other files where I tried parallelization and smaller scale memoization)

fn main() {
    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<u64> = raw_data
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // the count that we end up getting
    let mut count: u128 = 0;

    // The number of times that we need
    let times: usize = 75;

    // A memoization table
    // It holds the number, and how many times it should do it's thing
    let mut memo: HashMap<(u64, usize), u128> = HashMap::new();

    // For each thing in the input, we need to start the recurrence
    for number in data.iter() {
        count += find_length(&mut memo, *number, times);
    }

    // The length
    println!("There are {} stones", count);
}

// Okay, so what we want to do is make it run, then return the recurrence
fn find_length(memo: &mut HashMap<(u64, usize), u128>, number: u64, times_left: usize) -> u128 {
    // If the times left is zero, we're done
    if times_left == 0 {
        return 1u128;
    }

    // Checking the memoization table to see if we have to do any more calculations
    if let Some(length) = memo.get(&(number, times_left)) {
        return *length;
    }

    // Figuring out this one
    if number == 0 {
        // Finding length (heh see what I did there)
        let length = find_length(memo, 1, times_left - 1);

        // Updating memo table
        memo.insert((number, times_left), length);

        return length;
    }
    if number < 10 || num_digits(number) % 2 != 0 {
        // Don't worry, I won't pun this one
        let length = find_length(memo, number * 2024, times_left - 1);

        // Updating memo table
        memo.insert((number, times_left), length);

        return length;
    }

    let as_str: String = format!("{}", number);
    let length = find_length(
        memo,
        as_str[..(as_str.len() / 2)].parse::<u64>().unwrap(),
        times_left - 1,
    ) + find_length(
        memo,
        as_str[(as_str.len() / 2)..].parse::<u64>().unwrap(),
        times_left - 1,
    );

    // Updating memo table
    memo.insert((number, times_left), length);

    return length;
}

// Helper function to count digits
fn num_digits(mut n: u64) -> usize {
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:

fn read_contents(test: bool) -> String {
    let file_path = if test { "Testing.txt" } else { "Input.txt" };

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn _get_words(s: &String) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
