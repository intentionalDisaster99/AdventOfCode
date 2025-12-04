mod file_reader;
use crate::file_reader::{ get_lines, read_contents, read_csv };

fn main() {

    print!("First half answer: {}\n", first_half(false));
    print!("Second half answer: {}\n", second_half(false));

}

fn first_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

    let mut out: u128 = 0;

    let map = data.clone().iter().map(|e| e.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let dirs: Vec<[isize; 2]> = vec![
        [1, 1],
        [1, 0],
        [1, -1],
        [0, 1],
        [0, -1],
        [-1, 1],
        [-1, 0],
        [-1,-1]
    ];

    for (y, row) in data.iter().enumerate() {
        for (x, element) in row.chars().collect::<Vec<char>>().iter().enumerate() {
            if *element != '@' { continue; }
            let mut around = 0;

            for dir in dirs.iter() {

                // Finding new coordinates
                let new_x = x as isize + dir[1];
                let new_y = y as isize + dir[0];
                
                // Checking bounds
                if new_x < 0 || new_y < 0 {
                    continue;
                }
                if new_x as usize >= map[0].len() || new_y as usize >= map.len() {
                    continue;
                }

                // Actually checking the location 
                if map[new_y as usize][new_x as usize] == '@' {
                    around += 1;
                }
            }

            if around < 4 { out += 1; }

        }
    }

    out

}



fn second_half(test: bool) -> u128 {
    
    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

    let mut out: u128 = 0;

    let mut map = data.clone().iter().map(|e| e.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let dirs: Vec<[isize; 2]> = vec![
        [1, 1],
        [1, 0],
        [1, -1],
        [0, 1],
        [0, -1],
        [-1, 1],
        [-1, 0],
        [-1,-1]
    ];

    // If we didn't find any this iteration, we are done
    let mut current_round = 1;

    while current_round != 0 {

        current_round = 0;

        for (y, row) in map.clone().iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                if *element != '@' { continue; }
                let mut around = 0;

                for dir in dirs.iter() {

                    // Finding new coordinates
                    let new_x = x as isize + dir[1];
                    let new_y = y as isize + dir[0];
                    
                    // Checking bounds
                    if new_x < 0 || new_y < 0 {
                        continue;
                    }
                    if new_x as usize >= map[0].len() || new_y as usize >= map.len() {
                        continue;
                    }

                    // Actually checking the location 
                    if map[new_y as usize][new_x as usize] == '@' {
                        around += 1;
                    }
                }

                if around < 4 { 
                    out += 1;
                    current_round += 1;

                    // Updating map
                    map[y][x] = '.';
                }

            }
        }
    }

    out
}



// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_check() {
        assert_eq!(13, first_half(true));
    }
    
    #[test]
    fn second_half_check() {
        assert_eq!(43, second_half(true));
    }


}