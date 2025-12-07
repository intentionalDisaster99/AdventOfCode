// This makes a board of characters that we can easily change and read from and index nicely
// It is indexed simply as .get(Pos)


#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Board {
    board: Vec<Vec<char>>,
}





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


// Now for the position struct: a simple 2d vector
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pos {
    x: usize,
    y: usize,
}

// All of the methods, unless otherwise specified are designed to create a new position because it isn't a large amount of memory
impl Pos {

    pub fn new(x: usize, y: usize) -> Pos {
        Pos{x,y}
    }

    pub fn add(&self, other: Pos) -> Pos {
        Pos::new(self.x + other.x, self.y + other.y)
    }

    pub fn dot(&self, other: Pos) -> Pos {
        Pos::new(self.x * other.x, self.y * other.y)
    }
}