
use std::fmt;
use std::ops::{Add, Mul, Sub, Div};
use std::thread::current;


// -------------------------------- Simple Board --------------------------------

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Board<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}


#[allow(dead_code)]
impl<T> Board<T>
where
    T: Copy + PartialEq + fmt::Debug, // Constraints for ease of use
{
    /// Creates a new empty board with default values
    pub fn new(width: usize, height: usize, default_val: T) -> Board<T> {
        Board {
            data: vec![default_val; width * height],
            width,
            height,
        }
    }

    /// Helper to convert raw 2D vectors (like from parsing lines) into 1D
    pub fn from_2d(input: Vec<Vec<T>>) -> Board<T> {
        let height = input.len();
        let width = if height > 0 { input[0].len() } else { 0 };
        
        // Flatten the nested vectors into one
        let data = input.into_iter().flatten().collect();

        Board { data, width, height }
    }

    pub fn find(&self, target: T) -> Option<Pos> {
        // Iterate via index/enumerate, then calculate x/y from the index
        self.data.iter().position(|&x| x == target).map(|idx| {
            let y = idx / self.width;
            let x = idx % self.width;
            Pos::new(x, y)
        })
    }

    pub fn is_valid_location(&self, pos: &Pos) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    pub fn set(&mut self, pos: &Pos, val: T) {
        let idx = self.get_index(pos);
        self.data[idx] = val;
    }

    pub fn get(&self, pos: &Pos) -> T {
        self.data[self.get_index(pos)]
    }

    /// Internal helper to calculate 1D index from 2D coordinate
    #[inline(always)]
    fn get_index(&self, pos: &Pos) -> usize {
        pos.y * self.width + pos.x
    }
}

// Display implementation for Board
// Slices the 1D array into chunks of 'width' to print rows
impl<T: fmt::Display> fmt::Display for Board<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.chunks(self.width) {
            for item in row {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Debug implementation
impl<T: fmt::Debug> fmt::Debug for Board<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.chunks(self.width) {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

// -------------------------------- 2 Dimensional Positions --------------------------------
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

// All of the methods, unless otherwise specified are designed to create a new position because it isn't a large amount of memory
#[allow(dead_code)]
impl Pos {

    pub fn new(x: usize, y: usize) -> Pos {
        Pos{x,y}
    }

    pub fn add(&self, other: &Pos) -> Pos {
        Pos::new(self.x + other.x, self.y + other.y)
    }

    pub fn dot(&self, other: Pos) -> usize {
        self.x * other.x + self.y * other.y
    }

    pub fn mul(&self, scalar: usize) -> Pos {
        Pos::new(self.x * scalar, self.y * scalar)
    }

    pub fn div(&self, scalar: f64) -> Pos {
        Pos::new((self.x as f64 / scalar) as usize,(self.y as f64 / scalar) as usize)
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)

    }
}

// -------------------------------- 3 Dimensional Positions --------------------------------

#[derive(Clone, Debug, Hash, Eq, PartialEq)]

pub struct Pos3 {
    x: usize,
    y: usize,
    z: usize,
}

#[allow(dead_code)]
impl Pos3 {

    pub fn new(x: usize, y: usize, z: usize) -> Pos3 {
        Pos3{x, y, z}
    }

    pub fn add(&self, other: Pos3) -> Pos3 {
        Pos3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn dot(&self, other: Pos3) -> usize {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn mul(&self, scalar: usize) -> Pos3 {
        Pos3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn div(&self, scalar: f64) -> Pos3 {
        Pos3::new((self.x as f64 / scalar) as usize,(self.x as f64 / scalar) as usize, (self.z as f64 / scalar) as usize)
    }

}

impl std::fmt::Display for Pos3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


// -------------------------------- Simple Matrix --------------------------------
#[allow(dead_code)]
pub struct Matrix {
    pub num_rows: usize,
    pub num_cols: usize,
    pub data: Vec<f64>,
}

// Basic operations for any class/struct
impl Matrix {

    pub fn new(num_rows: usize, num_cols: usize, data: Vec<f64>) -> Matrix {
        Matrix {num_rows, num_cols, data}
    }

    pub fn from_vectors(input: &Vec<Vec<f64>>) -> Matrix {
        let data = input.iter().flatten().map(|a|*a).collect::<Vec<f64>>();
        Matrix::new(input.len(), input[0].len(), data)
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.num_cols + col]
    }

    pub fn set(&mut self, new_val: f64, row: usize, col: usize) -> &mut Self {
        self.data[row * self.num_cols + col] = new_val;
        self
    }

}

// Basic Operation implementation
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {

        todo!();
    }
}
impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {

        todo!();
    }
}impl Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {

        todo!();
    }
}

// Fun linear algebra stuff
impl Matrix {

    /*
        Pseudocode I stole from wikipedia
        row := 0 /* Initialization of the pivot row */
        col := 0 /* Initialization of the pivot column */ k

        while row ≤ m and col ≤ n:
            /* Find the col-th pivot: */
            pivot_row := argmax (i = row ... self.num_rows, abs(matrix[i, col]))
            if matrix[i_max, col] = 0:
                /* No pivot in this column, pass to next column */
                col := col + 1
            else:
                swap rows(row, pivot_row)
                /* Do for all rows below pivot: */
                for i = row + 1 ... m:
                    scalar := matrix[i, col] / matrix[row, col]
                    /* Fill with zeros the lower part of pivot column: */
                    matrix[i, col] := 0
                    /* Do for all remaining elements in current row: */
                    for j = col + 1 ... n:
                        matrix[i, j] := matrix[i, j] - matrix[row, j] * scalar
                /* Increase pivot row and column */
                row += 1
                col += 1 

     */

    pub fn row_echelon(&mut self) -> &mut Self {
        
        let mut row = 0;
        let mut col = 0;

        while row < self.num_rows && col < self.num_cols {

            // Finding this column's pivot (we can search the rest of this column continue down and to the right)
            let pivot_row = (row..self.num_rows).fold(0, |max_row_index, current_row_index| {
                if self.data[current_row_index * self.num_cols + col] > self.data[max_row_index * self.num_cols + col] {
                    return current_row_index
                }
                max_row_index
            });


            // If the maximum element we just found is zero, then we don't have a pivot in this column
            // If the one we found is behind the row we are on (meaning we didn't find one) we can skip too
            if self.data[pivot_row * self.num_cols + col] == 0.0 || pivot_row <= row {
                // On to the next column
                col += 1;
                continue;
            }

            // Swapping so that the row with the maximum value is where we want it
            self.swap_row(row, pivot_row);

            // For every row below where we found the pivot, we want to multiply by a scalar that causes it to go to zero when we add the current row
            for index_row_to_change in (row + 1)..self.num_rows {

                // Multiplying by the scalar
                let scalar = - self.data[index_row_to_change * self.num_cols + col] / self.data[row * self.num_cols + col];

                // Adding the current row to that one
                self.add_rows(index_row_to_change, row, scalar);

                // Making sure that everything directly below the pivot is exactly zero (this gets rid of floating point error)
                self.data[col + index_row_to_change * self.num_cols] = 0.0;

            }

            col += 1;
            row += 1;

        }    

        self

    }

    pub fn reduced_row_echelon(&mut self) -> &mut Self {
        
        let mut row = 0;
        let mut col = 0;

        while row < self.num_rows && col < self.num_cols {

            // println!("Row: {row}, Col: {col}: \n{self}");

            // Finding this column's pivot (we can search the rest of this column continue down and to the right)
            let pivot_row = (row..self.num_rows).fold(usize::MAX, |max_row_index, current_row_index| {
                if max_row_index == usize::MAX { return current_row_index; }
                if self.data[current_row_index * self.num_cols + col] > self.data[max_row_index * self.num_cols + col] {
                    return current_row_index
                }
                max_row_index
            });

            // println!("The pivot we have is at ")


            // If the maximum element we just found is zero, then we don't have a pivot in this column
            // If the one we found is behind the row we are on (meaning we didn't find one) we can skip too
            if pivot_row == usize::MAX || self.data[pivot_row * self.num_cols + col] == 0.0 {
                // On to the next column
                // println!("Didn't find a pivot in column {col}");
                col += 1;
                continue;
            } //else {println!("Pivot found at row: {pivot_row}, col: {col} of {}", self.data[pivot_row * self.num_cols + col]);}

            // Swapping so that the row with the maximum value is where we want it
            self.swap_row(row, pivot_row);

            // println!("Pivot is now moved to row {row}:\n{self}");

            // Converting the pivot to 1
            self.multiply_row(1.0/self.data[row * self.num_cols + col], row);

            // println!("Pivot should be 1:\n{self}");

            // For every row below where we found the pivot, we want to multiply by a scalar that causes it to go to zero when we add the current row
            for index_row_to_change in (row + 1)..self.num_rows {

                // Multiplying by the scalar
                let scalar = - self.data[index_row_to_change * self.num_cols + col] / self.data[row * self.num_cols + col];

                // Adding the current row to that one
                self.add_rows(index_row_to_change, row, scalar);

                // Making sure that everything directly below the pivot is exactly zero (this gets rid of floating point error)
                self.data[col + index_row_to_change * self.num_cols] = 0.0;

            }

            // println!("Only a pivot should be in column {col}\n{self}");

            // Make sure that there is no floating error by making the rest of the rows below the pivot zero

            col += 1;
            row += 1;

        }    

        // Going through and clearing out the rest to bring it to reduced echelon form
        row = self.num_rows - 1;
        col = self.num_cols - 1;
        loop {

            // Breaking if we have reached the bounds (row is 1 because we shouldn't need to play with the top row)
            if row == 0 || col == 0 {
                break;
            }

            // Searching for this row's pivot
            let mut pivot_col = usize::MAX;
            for i in 0..=col {
                if (self.data[i + self.num_cols * row] - 1.0).abs() < 1e-6 {
                    pivot_col = i;
                    break;
                }
            }
            
            // If we didn't find one, then we can go up a row
            if pivot_col == usize::MAX {
                row -= 1;
                if row == 2 { break; }
                continue;
            }
            
            // Making every element above this pivot equal zero
            for row_to_change in 0..row {
                self.add_rows(row_to_change, row, -self.data[row_to_change * self.num_cols + pivot_col]);
            }

            row -= 1;
            col -= 1;

        }

        self

    }
    
    // Solving with Gaussian elimination
    // Does not integerize or normalize the output vector (plugs in 1 for each free variable)
    pub fn solve(&self, b: Vec<f64>) -> Vec<f64> {
        if b.len() != self.num_rows {
            panic!("Sizes are not compatible! Could not solve.");
        }

        let mut out = vec![1.0; b.len()];
        
        // Now for each pivot we need to solve for (we assume everything else is 0.0)
        for col_index in 0..self.num_cols {

            // Finding the row_index of the pivot in this column 
            let mut row_index = usize::MAX;
            for i in (self.num_rows - 1)..=0 {
                if (self.data[col_index + self.num_cols * i] - 1.0).abs() < 1e-6 {
                    row_index = i;
                    break;
                }
            }

            // If we didn't find a pivot, then we can just continue on to the next column
            if row_index == usize::MAX {
                continue;
            }

            // We have a pivot, so we can play with this row
            let mut total = b[row_index] + 1.0;
            for i in 0..self.num_cols {
                total -= self.data[i + self.num_cols * row_index];
            }
            out[row_index] = total;

        }
        // As of right now, this only returns ones

        out

    }

    pub fn transpose(&mut self) -> &mut Self {
        let mut new_data = vec![0.0; self.num_cols * self.num_rows];
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                // println!("{} -> {}", row * self.num_rows + col, col * self.num_rows + row);
                (self.data[row * self.num_cols + col], new_data[col * self.num_rows + row]) = (new_data[col * self.num_rows + row], self.data[row * self.num_cols + col]);  
            }
        }
        self.data = new_data;
        (self.num_rows, self.num_cols) = (self.num_cols, self.num_rows);
        self
    }

    // pub fn transposed(&self) -> &mut Self {
    //     println!("-----------------------------------------------------\nBefore:\n{}", self);
    //     let mut out_vector = vec![vec![0.0; self.num_cols]; self.num_rows];
    //     for row in 0..self.num_rows {
    //         for col in 0..self.num_cols {
    //             (self.data[row * self.num_rows + col], self.data[col * self.num_cols + row]) = (self.data[col * self.num_cols + row], self.data[row * self.num_rows + col]);
    //         }
    //     }
    //     println!("After:\n{}", out);
    //     self
    // }

}

// Row operations
impl Matrix {

    pub fn multiply_row(&mut self, scalar: f64, row_num: usize) -> &mut Self {
        for i in (row_num * self.num_cols)..((row_num + 1) * self.num_cols) {
            self.data[i] *= scalar
        }
        self
    }

    pub fn swap_row(&mut self, row_a: usize, row_b: usize) -> &mut Self {
        for i in 0..self.num_cols {
            (self.data[self.num_cols * row_a + i], self.data[self.num_cols * row_b + i]) = (self.data[self.num_cols * row_b + i], self.data[self.num_cols * row_a + i]);
        }
        self
    }

    pub fn add_rows(&mut self, row_to_add_to: usize, row_to_add: usize, scalar: f64) -> &mut Self {
        for i in 0..self.num_cols {
            self.data[self.num_cols * row_to_add_to + i] += self.data[self.num_cols * row_to_add + i] * scalar; 
        }
        self
    }

}


// Displaying in a pretty way
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let max_length = {
            let mut max_length = 0;
            for element in self.data.iter() {
                max_length = format!( "{:.3}, ", element).len().max(max_length)
            }
            max_length
        };

        let col_offset = self.num_cols * max_length + (self.num_cols - 1) * 2;
        
        for row in 0..self.num_rows {

            // If the last or first, we can print a corner piece
            if row == 0 {
                writeln!(f, "┌ { :^col_offset$} ┐", "")?;
                write!(f, "│ ")?;
            } else {
                write!(f, "│ ")?;
            }

            // Printing the actual data
            for col in 0..self.num_cols {
                write!(f, "{:>max_length$.3}", self.data[col + self.num_cols * row])?;
                
                // We only want the comma if this isn't the last column
                if col != self.num_cols - 1 {
                    write!(f, ", ")?;
                } 
            }

           writeln!(f, " │")?;

        }

        // The bottom of the matrix
        writeln!(f, "└ { :^col_offset$} ┘", "")?;

        Ok(())
    }
}

