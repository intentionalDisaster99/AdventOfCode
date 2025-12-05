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

    let mut out: u128 = 0;

    // Separating them 
    let mut ranges_strings: Vec<String> = vec![];
    let mut ids_strings: Vec<String> = vec![];

    let mut found= false;
    for line in data.iter() {
        if *line == "".to_string() {
            found = true;
            continue;
        }
        if !found {
            ranges_strings.push(line.clone());
        } else {
            ids_strings.push(line.clone());
        }
    }

    let ranges = ranges_strings.iter().map(
        |range| range.split("-").map(
            |e| e.to_string().parse::<u128>().unwrap()
        ).collect::<Vec<u128>>()
    ).collect::<Vec<Vec<u128>>>();

    let ids = ids_strings.iter().map(|num| num.parse::<u128>().expect("Bad ID parsing error")).collect::<Vec<u128>>();

    'id_loop: for id in ids.iter() {
        for range in ranges.iter() {
            if range[0] <= *id && range[1] >= *id {
                out += 1;
                continue 'id_loop;
            }
        }
    } 

    out
}

fn second_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

    let mut ranges_strings: Vec<String> = vec![];

    for line in data.iter() {
        if *line == "".to_string() { break; }
        ranges_strings.push(line.clone());
    }
    let mut ranges = ranges_strings.iter().map(
        |range| range.split("-").map(
            |e| e.to_string().parse::<u128>().unwrap()
        ).collect::<Vec<u128>>()
    ).collect::<Vec<Vec<u128>>>();

    let mut out: u128 = 0;

    // Sorting so I have to do less checking later
    ranges.sort_by(|a, b| a[0].cmp(&b[0]));

    // Starting at the first range
    let mut current_start = ranges[0][0];
    let mut current_end = ranges[0][1];

    // Merging ranges
    for range in ranges.iter().skip(1) {
        let next_start = range[0];
        let next_end = range[1];

        // If they are touching, we can treat them as continuous
        if next_start <= current_end + 1 {
            if next_end > current_end {
                current_end = next_end;
            }
        } else {
            // They don't overlap, so we can add the completed range and continue
            out += current_end - current_start + 1;

            current_start = next_start;
            current_end = next_end;
        }
    }

    // Adding the final range
    out += current_end - current_start + 1;

    out 
}


// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_check() {
        assert_eq!(3, first_half(true));
    }
    
    #[test]
    fn second_half_check() {
        assert_eq!(14, second_half(true));
    }


}