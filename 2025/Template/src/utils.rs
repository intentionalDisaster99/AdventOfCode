
use std::fmt;
use std::ops::{Add, Mul, Sub, Div};


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
        h := 1 /* Initialization of the pivot row */
k := 1 /* Initialization of the pivot column */

while h ≤ m and k ≤ n:
    /* Find the k-th pivot: */
    i_max := argmax (i = h ... m, abs(A[i, k]))
    if A[i_max, k] = 0:
        /* No pivot in this column, pass to next column */
        k := k + 1
    else:
        swap rows(h, i_max)
        /* Do for all rows below pivot: */
        for i = h + 1 ... m:
            f := A[i, k] / A[h, k]
            /* Fill with zeros the lower part of pivot column: */
            A[i, k] := 0
            /* Do for all remaining elements in current row: */
            for j = k + 1 ... n:
                A[i, j] := A[i, j] - A[h, j] * f
        /* Increase pivot row and column */
        h := h + 1
        k := k + 1 

     */

    pub fn row_reduce(&mut self) -> Self {
        
        // We want to get the top left to be one, 
        let divisor = self.get(0, 0);

        todo!();

    }

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

