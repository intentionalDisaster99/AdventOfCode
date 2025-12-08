mod file_reader;
mod board;
use crate::file_reader::*;
use crate::board::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::thread::current;
use std::cmp::Reverse;
use priority_queue::PriorityQueue;


fn main() {



    print!("First half answer: {}\n", first_half(false));
    print!("Second half answer: {}\n", second_half(false));

}

fn first_half(test: bool) -> u128 {

    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);
    let number_of_connections: usize = { if test {10} else {1000} };

    // Parsing the strings into locations
    let positions: Vec<(usize, usize, usize)> = data.iter().map(
        |row|  
        row.split(",").map(
            |num|
            num.parse::<usize>().expect(&format!("bad unwrap {}", num))
        ).collect_tuple().expect("Bad sizing for the input")
    ).collect();

    // Figure out which ones are the closest
    let mut closest: PriorityQueue<((usize, usize, usize), (usize, usize, usize)), Reverse<usize>> = PriorityQueue::new();
    for (i, this) in positions.iter().enumerate() {
        for other in positions.clone().iter().skip(i+1) {
            closest.push((*this, *other), Reverse(dist(this, other)));
        }
    }

    // Now we just connect the closest ones
    let mut circuits: Vec<Vec<(usize, usize, usize)>> = vec![];
    // 'main_circuit_loop: while current_number_of_connections < number_of_connections {
    'main_circuit_loop: for _ in 0..number_of_connections {
        
        let current_connection = closest.pop().unwrap();

        // Checking to see if they are already connected in any of our connections
        let mut circuit_indices: Vec<usize> = vec![];
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&current_connection.0.0) && circuit.contains(&current_connection.0.1) {
                // They are already connected, we don't need to do anything continue on 
                continue 'main_circuit_loop;
            }

            // If one is in this circuit, we need to note which one it is to come back to after the loop
            if circuit.contains(&current_connection.0.0) || circuit.contains(&current_connection.0.1) {
                circuit_indices.push(i);
            }

        }

        // If we found that each are in different circuits, we need to merge them
        if circuit_indices.len() == 2 {
            circuits[circuit_indices[0]] = [&circuits[circuit_indices[0]][..], &circuits[circuit_indices[1]][..]].concat();
            circuits.remove(circuit_indices[1]);
        }

        // If we found only one pre-existing circuit, we can just add the other one
        if circuit_indices.len() == 1 {
            if circuits[circuit_indices[0]].contains(&current_connection.0.0) {
                circuits[circuit_indices[0]].push(current_connection.0.1);
            } else {
                circuits[circuit_indices[0]].push(current_connection.0.0);
            };
        }

        // If we didn't find any, we need to make a new one
        if circuit_indices.len() == 0 {
            circuits.push(vec![current_connection.0.0, current_connection.0.1]);

        }

        if circuit_indices.len() > 2 {
            panic!("Dafuq, circuit_indicies has a length of {}", circuit_indices.len()); 
        }

    }

    // Now to sort by length and multiply the first three
    circuits.sort_by(|a, b| (b.len()).cmp(&a.len()));

    return (circuits[0].len() * circuits[1].len() * circuits[2].len()) as u128;

}


// This is currently distance squared for a minor speedup
fn dist(a: &(usize, usize, usize), b: &(usize, usize, usize)) -> usize {
    let dx = (a.0 as isize - b.0 as isize).abs() as usize;
    let dy = (a.1 as isize - b.1 as isize).abs() as usize; 
    let dz = (a.2 as isize - b.2 as isize).abs() as usize;
    dx*dx + dy*dy + dz*dz

}
    

fn second_half(test: bool) -> u128 {
    
    let raw_data: String = read_contents(test);
    let data: Vec<String> = get_lines(&raw_data);

     // Parsing the strings into locations
    let positions: Vec<(usize, usize, usize)> = data.iter().map(
        |row|  
        row.split(",").map(
            |num|
            num.parse::<usize>().expect(&format!("bad unwrap {}", num))
        ).collect_tuple().expect("Bad sizing for the input")
    ).collect();

    // Figure out which ones are the closest
    let mut closest: PriorityQueue<((usize, usize, usize), (usize, usize, usize)), Reverse<usize>> = PriorityQueue::new();
    for (i, this) in positions.iter().enumerate() {
        for other in positions.clone().iter().skip(i+1) {
            closest.push((*this, *other), Reverse(dist(this, other)));
        }
    }

    // To note down what the last two were
    let mut last: [usize; 2] = [0; 2];

    // Now we just connect the closest ones
    let mut circuits: Vec<Vec<(usize, usize, usize)>> = vec![];
    'main_circuit_loop: loop {

        if closest.len() == 0 { break; }
        
        let current_connection = closest.pop().unwrap();

        // Checking to see if they are already connected in any of our connections
        let mut circuit_indices: Vec<usize> = vec![];
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&current_connection.0.0) && circuit.contains(&current_connection.0.1) {
                // They are already connected, we don't need to do anything continue on 
                continue 'main_circuit_loop;
            }

            // If one is in this circuit, we need to note which one it is to come back to after the loop
            if circuit.contains(&current_connection.0.0) || circuit.contains(&current_connection.0.1) {
                circuit_indices.push(i);
            }

        }

        // If we found that each are in different circuits, we need to merge them
        if circuit_indices.len() == 2 {
            circuits[circuit_indices[0]] = [&circuits[circuit_indices[0]][..], &circuits[circuit_indices[1]][..]].concat();
            circuits.remove(circuit_indices[1]);
            last = [current_connection.0.0.0, current_connection.0.1.0];
        }

        // If we found only one pre-existing circuit, we can just add the other one
        if circuit_indices.len() == 1 {
            if circuits[circuit_indices[0]].contains(&current_connection.0.0) {
                circuits[circuit_indices[0]].push(current_connection.0.1);
                last = [current_connection.0.0.0, current_connection.0.1.0];
            } else {
                circuits[circuit_indices[0]].push(current_connection.0.0);
                last = [current_connection.0.0.0, current_connection.0.1.0];
            };
        }

        // If we didn't find any, we need to make a new one
        if circuit_indices.len() == 0 {
            circuits.push(vec![current_connection.0.0, current_connection.0.1]);
            last = [current_connection.0.0.0, current_connection.0.1.0];
        }

        if circuit_indices.len() > 2 {
            panic!("Dafuq, circuit_indicies has a length of {}", circuit_indices.len()); 
        }

    }

    return (last[0] * last[1]) as u128;
}

// Unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half_check() {
        let answer: Option<u128> = Some(40);
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, first_half(true)),
        }
    }
    
    #[test]
    fn second_half_check() {
        let answer: Option<u128> = Some(25272);
        match answer {
            None => panic!("Not yet added"),
            Some(a) => assert_eq!(a, second_half(true)),
        }
    }


}


/*


 // The two closest to each other followed by their distance
        let mut closest: ((usize, usize, usize), (usize, usize, usize), usize) = ((0, 0, 0), (0, 0, 0), usize::MAX);

        // Iterating across all of the positions to find the ones that are closest to each other 
        for junction_box in positions.iter() {

            // Checking the distance between all of the others
            // Yes, I know this is slow, but I don't want to look up solutions to the traveling salesman problem yet
            'connection_finder: for other in positions.iter() {

                let distance = dist(other, junction_box);

                // Checking to make sure they aren't both in one of the other connections
                for circuit in all_circuits.iter() {
                    let mut is_to: u8 = 0;
                    for junction in circuit.iter() {
                        if junction == other || junction == junction_box {
                            is_to += 1;
                        }
                    }
                    if is_to == 2 { continue 'connection_finder; }
                }

                if distance > closest.2 {
                    closest = (
                        *junction_box,
                        *other,
                        distance,
                    )
                }

            }

        }

        // Now to push that one we found into any */