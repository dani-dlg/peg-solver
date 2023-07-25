use std::fmt::Debug;

const BOARD_SIZE: usize = 7;
const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    U, D, L, R
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Invalid,
    Occupied
}

#[derive(Copy, Clone)]
pub struct Move (i8, i8, Dir);

impl Move {
    pub fn parse_move(input: &str) -> Result<Self, String> {
        if input.len() != 5 {
            return Err(String::from("Invalid length"));
        }
        let x = ALPHABET.chars().position(|c| c == input.chars().next().expect("The input lenght should be 3"))
            .ok_or("The first char wasn't a valid letter")?;
        let y = input.chars().nth(1).expect("The input lenght should be 3")
            .to_digit(10).ok_or("The second character wasn't a valid digit")?;
        if  y >= BOARD_SIZE as u32 {
            return Err(String::from("The second number exceeds the board size!"));
        }
        let dir = input.chars().nth(2).expect("The input lenght should be 3");
        let dir = match dir {
            'U' => Some(Dir::U),
            'D' => Some(Dir::D),
            'L' => Some(Dir::L),
            'R' => Some(Dir::R),
            _ => None
        };
        let dir = dir.ok_or("Direction must be U, D, L or R")?;
        
        Ok(Move(x as i8, y as i8, dir))
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Move").field(&ALPHABET.chars().nth(self.0 as usize).unwrap()).field(&self.1).field(&self.2).finish()
    }
}

#[derive(Debug)]
pub struct Board {
    board: [[CellType; BOARD_SIZE]; BOARD_SIZE],
    pub score: i32,
    pub moves: Vec<Move>
}   

impl Board {
    pub fn from_strings(s: &[&str]) -> Self {
        let mut board = Self::default();

        for (y, row) in s.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                board.board[y][x] = match c {
                    'o' => CellType::Occupied,
                    'x' => CellType::Empty,
                    _ => CellType::Invalid
                }
            }
        }
        board
    }

    pub fn is_move_valid(&self, mov: Move) -> bool {
        let (x, y, dir) = (mov.0, mov.1, mov.2);
        match dir {
            Dir::D => self.cell_at_pos_is(x, y + 2, CellType::Empty) && self.cell_at_pos_is(x, y + 1, CellType::Occupied) && self.cell_at_pos_is(x, y, CellType::Occupied),
            Dir::U => self.cell_at_pos_is(x, y - 2, CellType::Empty) && self.cell_at_pos_is(x, y - 1, CellType::Occupied) && self.cell_at_pos_is(x, y, CellType::Occupied),
            Dir::L => self.cell_at_pos_is(x - 2, y, CellType::Empty) && self.cell_at_pos_is(x - 1, y, CellType::Occupied) && self.cell_at_pos_is(x, y, CellType::Occupied),
            Dir::R => self.cell_at_pos_is(x + 2, y, CellType::Empty) && self.cell_at_pos_is(x + 1, y, CellType::Occupied) && self.cell_at_pos_is(x, y, CellType::Occupied),
        }
    }

    /// returns false if the given position is out of bounds
    fn cell_at_pos_is(&self, x: i8, y: i8, cell_type: CellType) -> bool {
        !self.is_oob(x, y) && self.board[y as usize][x as usize] == cell_type
    }

    fn is_oob(&self, x: i8, y: i8) -> bool {
        y < 0 || x < 0 || x >= BOARD_SIZE as i8 || y >= BOARD_SIZE as i8
    }

    /// PANICS if the given move is invalid, please check with is_move_valid first. 
    pub fn make_move(&mut self, mov: Move) {
        let (x, y, dir) = (mov.0, mov.1, mov.2);
        #[allow(clippy::identity_op)]
        match dir {
            Dir::D => {
                self.set_cell_at(x, y + 2, CellType::Occupied);
                self.set_cell_at(x, y + 1, CellType::Empty);
                self.set_cell_at(x, y + 0, CellType::Empty);
            }
            Dir::U => {
                self.set_cell_at(x, y - 2, CellType::Occupied);
                self.set_cell_at(x, y - 1, CellType::Empty);
                self.set_cell_at(x, y - 0, CellType::Empty);
            }
            Dir::L => {
                self.set_cell_at(x - 2, y, CellType::Occupied);
                self.set_cell_at(x - 1, y, CellType::Empty);
                self.set_cell_at(x - 0, y, CellType::Empty);
            }
            Dir::R => {
                self.set_cell_at(x + 2, y, CellType::Occupied);
                self.set_cell_at(x + 1, y, CellType::Empty);
                self.set_cell_at(x + 0, y, CellType::Empty);
            }
        };
        self.score += 1;
        self.moves.push(mov);
    }

    pub fn undo_move(&mut self, mov: Move) {
        let (x, y, dir) = (mov.0, mov.1, mov.2);
        #[allow(clippy::identity_op)]
        match dir {
            Dir::D => {
                self.set_cell_at(x, y + 2, CellType::Empty);
                self.set_cell_at(x, y + 1, CellType::Occupied);
                self.set_cell_at(x, y + 0, CellType::Occupied);
            }
            Dir::U => {
                self.set_cell_at(x, y - 2, CellType::Empty);
                self.set_cell_at(x, y - 1, CellType::Occupied);
                self.set_cell_at(x, y - 0, CellType::Occupied);
            }
            Dir::L => {
                self.set_cell_at(x - 2, y, CellType::Empty);
                self.set_cell_at(x - 1, y, CellType::Occupied);
                self.set_cell_at(x - 0, y, CellType::Occupied);
            }
            Dir::R => {
                self.set_cell_at(x + 2, y, CellType::Empty);
                self.set_cell_at(x + 1, y, CellType::Occupied);
                self.set_cell_at(x + 0, y, CellType::Occupied);
            }
        };
        self.moves.remove(self.moves.len() - 1);
        self.score -= 1;
    }

    fn set_cell_at(&mut self, x: i8, y: i8, cell_type: CellType) {
        // eprintln!("Setcell called with x:{} y:{}", x, y);
        self.board[y as usize][x as usize] = cell_type;
    }

    pub fn valid_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.cell_at_pos_is(x as i8, y as i8, CellType::Invalid) {
                    continue;
                }
                for dir in &[Dir::U, Dir::D, Dir::L, Dir::R] {
                    if self.is_move_valid(Move(x as i8, y as i8, *dir)) {
                        moves.push(Move(x as i8, y as i8, *dir));
                    }
                }
            }
        }
        moves
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity((BOARD_SIZE + 2) * (BOARD_SIZE + 1));
        // print a header with the column numbers
        result.push_str("  ");
        for col in 0..BOARD_SIZE {
            result.push(ALPHABET.chars().nth(col).expect("Board size shouldn't exceed 27 rows")); 
        }
        result.push('\n');

        for y in 0..BOARD_SIZE {
            result.push_str(&y.to_string()); // <- unnecessary heap alloc? 
            result.push('|');
            for x in 0..BOARD_SIZE {
                result.push(
                    match self.board[y][x] {
                        CellType::Empty => '·',
                        CellType::Invalid => ' ',
                        CellType::Occupied => 'O'
                    }
                );
            }
            result.push('\n');
        }
        result
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [[CellType::Empty; BOARD_SIZE]; BOARD_SIZE],
            score: 0,
            moves: Vec::new(),
        }
    }
}