mod file_reader;
use crate::file_reader::{ read_contents, get_lines };


fn main() {

    println!("First half answer: ");
    first_half();
    println!("Second half answer: ");
    second_half();

}

fn first_half() {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<String> = get_lines(&raw_data);

    // We start with 50
    let mut value: i16 = 50;

    // The combination
    let mut sum: u128 = 0;

    for command in data.iter() {

        turn(&mut value, command);

        if value == 0 { sum += 1; }

    }

    println!("The combination is {}", sum);
}


fn turn(value: &mut i16, command: &String) {

    let mut dir: i16= 1; // We assume that it is going right

    // What direction are we turning it? 
    if command.chars().nth(0).expect("Found a command that had no size") == 'L' {
        dir = -1;
    } else if command.chars().nth(0).expect("Found a command that had no size") != 'R' {
        panic!("Bad command! Me no likey");
    }

    // Now to figure out how many times we turn
    let times: i16  = command[1..].parse().expect("Could not parse command");

    // Moving
    *value += dir * times;

    // Making sure there is no underflow
    while *value < 0 { *value += 100; }

    // Making sure there is no overflow
    while *value > 99 { *value -= 100; }
    

}



fn second_half() {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<String> = get_lines(&raw_data);

    // We start with 50
    let mut value: i16 = 50;

    // The combination
    let mut sum: u128 = 0;

    for command in data.iter() {

        sum += smart_turn(&mut value, command);

    }

    println!("The combination is {}", sum);
}


fn smart_turn(value: &mut i16, command: &String) -> u128 {

    let mut dir: i16= 1; // We assume that it is going right

    // What direction are we turning it? 
    if command.chars().nth(0).expect("Found a command that had no size") == 'L' {
        dir = -1;
    } else if command.chars().nth(0).expect("Found a command that had no size") != 'R' {
        panic!("Bad command! Me no likey");
    }

    let mut out = 0;

    for _ in 0..command[1..].parse().expect("Could not parse command") {

        *value += dir;

        // Making sure there is no underflow
        if *value < 0 { *value += 100; }

        // Making sure there is no overflow
        if *value > 99 { *value -= 100; }

        if *value == 0 { out += 1; }

    }

    out
}
