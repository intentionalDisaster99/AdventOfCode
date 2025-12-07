mod file_reader;
use std::collections::HashMap;

use crate::file_reader::{ get_lines, read_contents, read_csv };

fn main() {

    print!("First half answer: {}\n", first_half(false));
    print!("Second half answer: {}\n", second_half(false));

}

fn first_half(test: bool) -> u128 {
    
    let raw_data: String = read_contents(test);
    let data: Vec<Vec<char>> = get_lines(&raw_data).iter().map(|line| line.chars().collect()).collect(); // Indexed as [y][x]
    let mut out: u128 = 0;
    let mut current_pos: Option<Pos> = None;

    // Finding where we start
    for (y, row) in data.iter().enumerate() {
        for (x, element) in row.iter().enumerate() {
            if *element == 'S' {
                current_pos = Some(Pos {x: x, y: y});
            }
        }
    }

    // The buffer of tachyons, so that we can continually update them
    let mut tachyons: Vec<Pos> = vec![current_pos.expect("Did not find start")];

    'outer_loop: loop {

        // Moving down 
        for tachyon in tachyons.iter_mut() {
            tachyon.y += 1;
        }

        // Breaking if we are at the bottom
        if tachyons[0].y == data.len() { break 'outer_loop; }

        for (i, tachyon) in tachyons.clone().iter().enumerate().rev() {

            // Splitting
            if data[tachyon.y][tachyon.x] == '^' {

                out += 1;

                // Adding one left if we don't already have one
                if !contains_x_val(tachyon.x - 1, &tachyons.clone()) {
                    tachyons.push(Pos{x: tachyon.x - 1, y: tachyon.y});
                }

                // Right 
                if !contains_x_val(tachyon.x + 1, &tachyons) {
                    tachyons.push(Pos{x: tachyon.x + 1, y: tachyon.y});
                }

                // Removing the one that we already had
                tachyons.remove(i);

            }

        }

        // Making sure we don't have any clashing 
        for (i, tachyon) in tachyons.clone().iter().enumerate().rev() {
            for other in tachyons.clone().iter().skip(i+1).rev() {
                if other.x == tachyon.x {
                    tachyons.remove(i);
                }
            }
        }

    }

    // return tachyons.len() as u128
    out
}


fn contains_x_val(x: usize, tachyons: &Vec<Pos>) -> bool {

    // If it is too big or little to add, we just say there is already one there
    if x < 0 || x >= tachyons.len() { return false; }

    for t in tachyons.iter() {
        if t.x == x {
            return true;
        }
    }
    false
}


fn second_half(test: bool) -> u128 {

  
    let raw_data: String = read_contents(test);
    let data: Vec<Vec<char>> = get_lines(&raw_data).iter().map(|line| line.chars().collect()).collect(); // Indexed as [y][x]
    let mut out: u128 = 0;
    let mut start: Option<Pos> = None;

    // Finding where we start
    for (y, row) in data.iter().enumerate() {
        for (x, element) in row.iter().enumerate() {
            if *element == 'S' {
                start = Some(Pos {x: x, y: y});
            }
        }
    }

    // Recursively finding the number
    return rain(start.expect("Could not find start"), &data);

}

fn rain(pos: Pos, map: &Vec<Vec<char>>) -> u128 {
    let mut memo: HashMap<Pos, u128> = HashMap::new();
    rain_recursive(pos, map, &mut memo)
}

fn rain_recursive(input_pos: Pos, map: &Vec<Vec<char>>, memo: &mut HashMap<Pos, u128> ) -> u128 {

    if let Some(&cached_val) = memo.get(&input_pos.clone()) {
        return cached_val;
    }
    println!("At {}/{}", input_pos.y, map.len());

    // Moving down
    let mut pos = input_pos.clone();
    pos.y += 1;

    // Making sure we aren't off the map
    if pos.x == map[0].len() {
        memo.insert(pos.clone(), 0);
        return 0;
    }

    // Checking to see if we hit the bottom
    if pos.y == map.len() {
        memo.insert(pos.clone(), 1);
        return 1;
    }

    // Checking to see if we need to split
    if map[pos.y][pos.x] == '^' {
        let out = rain_recursive(Pos {x: pos.x + 1, y: pos.y}, map, memo) + if pos.x != 0 { rain_recursive(Pos {x: pos.x - 1, y: pos.y}, map, memo)} else {0};
        memo.insert(pos.clone(), out);
        return out
    }

    let out = rain_recursive(pos.clone(), map, memo);
    memo.insert(pos.clone(), out);
    return out
}


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_check() {
        let answer: Option<u128> = Some(21);
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, first_half(true)),
        }
    }
    
    #[test]
    fn second_half_check() {
        let answer: Option<u128> = Some(40);
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, second_half(true)),
        }
    }


}

