// OOP is so not necessary here

fn main() {
    // First, we have to read the data (true == test, false == input)
    let raw_data: String = read_contents(false);
    let data: Vec<Vec<char>> = get_lines(&raw_data);

    // We loop over every plant in the plot and make a plant type for each
    let mut plants: Vec<Vec<Plant>> = data
        .iter()
        .map(|row| {
            row.iter()
                .map(|plant_type| Plant {
                    perimeter: 0,
                    plant: *plant_type,
                    counted: false,
                })
                .collect::<Vec<Plant>>()
        })
        .collect::<Vec<Vec<Plant>>>();

    // Updating each plant and then plugging them in to the perimeters vector
    for (i, row) in plants.clone().iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            // The input of the surrounding plants
            let mut input = [None; 4];
            if j + 1 < row.len() {
                input[0] = Some(row[j + 1]);
            }
            if j != 0 {
                input[1] = Some(row[j - 1]);
            }
            if i + 1 < plants.len() {
                input[2] = Some(plants.clone()[i + 1][j]);
            }
            if i != 0 {
                input[3] = Some(plants.clone()[i - 1][j]);
            }

            // Finding and saving the value
            plants[i][j].get_perimeter(input);
        }
    }

    // The answer
    let mut price: u64 = 0;

    // Finding the regions
    for (i, row) in plants.clone().iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            // If this one is valid, we find the rest in the region
            if plants[i][j].plant != '.' {
                // Getting and adding
                let (area, perim) = get_area_perim(plants[i][j].plant, (i, j), &mut plants);
                price += area * perim;
            }
        }
    }

    println!("it will cost {} ($? Idk they don't mention it)", price);
}

// This is a recursive function that returns the (area, perimeter)
fn get_area_perim(
    plant_type: char,
    location: (usize, usize),
    plants: &mut Vec<Vec<Plant>>,
) -> (u64, u64) {
    // Skipping if this one is not the right type
    if plants[location.0][location.1].plant != plant_type {
        return (0, 0);
    }

    // Marking this one so we don't come back to it
    plants[location.0][location.1].plant = '.';

    let mut output: (u64, u64) = (1, plants[location.0][location.1].perimeter as u64);

    // Adding the ones around it
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    for dir in directions.iter() {
        // Calculating the position and throwing it out if it doesn't fit
        let pos: (isize, isize) = (
            location.0 as isize - dir.0 as isize,
            location.1 as isize - dir.1 as isize,
        );
        if pos.0 < 0
            || pos.1 < 0
            || pos.0 >= plants.len() as isize
            || pos.1 >= plants[0].len() as isize
        {
            continue;
        }

        // Making sure we haven't already visited this one
        if plants[pos.0 as usize][pos.1 as usize].plant == '.' {
            continue;
        }

        // Adding the value from the new location to the output
        let (one, two) = get_area_perim(plant_type, (pos.0 as usize, pos.1 as usize), plants);
        output.0 += one;
        output.1 += two;
    }

    output
}

// I am finally going to do some struct and stuff
#[derive(Clone, Copy)]
pub struct Plant {
    pub perimeter: u8,
    pub plant: char,

    // Lets us know whether or not this one has already been counted
    // This way, we don't repeat calculations on one region
    pub counted: bool,
}

// Function to find the perimeter given the surrounding plants
impl Plant {
    fn get_perimeter(&mut self, others: [Option<Plant>; 4]) -> u8 {
        let mut count: u8 = 0;
        for other in others.iter() {
            if other.is_none() {
                count += 1;
                continue;
            }
            if other.unwrap().plant != self.plant {
                count += 1;
            }
        }
        self.perimeter = count;
        count
    }
}

// Stuff to read the input
use std::fs;

// This will likely just be copied to every single other challenge:

fn read_contents(test: bool) -> String {
    let file_path = if test { "Testing.txt" } else { "Input.txt" };
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn get_lines(s: &String) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
