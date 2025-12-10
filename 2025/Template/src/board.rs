// This makes a board of characters that we can easily change and read from and index nicely
// It is indexed simply as .get(Pos)


#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Board {
    pub board: Vec<Vec<char>>,
}



#[allow(dead_code)]
impl Board {

    pub fn new(data: Vec<Vec<char>>) -> Board {
        Board {board: data}
    }

    pub fn find(&self, c: char) -> Option<Pos> {
        for (y, row) in self.board.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                if *element == c {
                    return Some(Pos::new(x, y));
                }
            }
        }
        None
    }

    pub fn is_valid_location(&self, pos: &Pos) -> bool {
        if pos.y >= self.board.len() || pos.x >= self.board[0].len() { return false; }
        true
    }

    pub fn set(&mut self, pos: &Pos, val: char) {
        // We are assuming they already checked bounds
        self.board[pos.y][pos.x] = val;
    }

    pub fn get(&self, pos: &Pos) -> char {
        // We are assuming they already checked bounds
        self.board[pos.y][pos.x]
    }

}

impl std::fmt::Display for Board {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.board {
            for c in row.iter() {
                write!(f, "{}", c)?; 
            }
            writeln!(f, "")?; 

        }
        Ok(())
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.board {
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
        Pos::new((self.x as f64 / scalar) as usize,(self.x as f64 / scalar) as usize)
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

}

impl std::fmt::Display for Pos3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


