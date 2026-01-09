// This makes a board of characters that we can easily change and read from and index nicely
// It is indexed simply as .get(Pos)
use std::fmt;

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


// Now for the position struct: a simple 2D vector
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

// Same thing as the position struct but for 3D positions (thank you 2025-8)
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


