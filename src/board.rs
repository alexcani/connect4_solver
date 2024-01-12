use std::fmt::Display;

use strum_macros::{EnumIter, FromRepr, EnumCount};

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum Slot {
    #[default] Empty,
    P1,
    P2
}

#[derive(Copy, Clone, PartialEq, Debug, EnumIter, FromRepr, EnumCount)]
pub enum Column {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G
}

impl From<char> for Column {
    fn from(c: char) -> Self {
        match c {
            '1' | 'A' | 'a' => Column::A,
            '2' | 'B' | 'b' => Column::B,
            '3' | 'C' | 'c' => Column::C,
            '4' | 'D' | 'd' => Column::D,
            '5' | 'E' | 'e' => Column::E,
            '6' | 'F' | 'f' => Column::F,
            '7' | 'G' | 'g' => Column::G,
            _ => panic!("Invalid column")
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Board {
    slots: [[Slot; WIDTH]; HEIGHT],
    height: [usize; WIDTH],  // Number of pieces in each column
    n_moves: u32
}

impl Board {
    /// Creates a new, empty board
    pub fn new() -> Board {
        Board {
            slots: [[Slot::Empty; WIDTH]; HEIGHT],
            height: [0; WIDTH],
            n_moves: 0
        }
    }

    /// Create a new board from a given position, using the sequence of played columns notation
    ///
    /// Example:
    /// ```
    /// # use connect4_solver::board::*;
    /// let board = Board::from_notation("32164625");
    /// ```
    /// This will create a board with the following layout (X is P1, O is P2)
    /// ```text
    /// - - - - - - -
    /// - - - - - - -
    /// - - - - - - -
    /// - - - - - - -
    /// - X - - - O -
    /// X O X X O O -
    /// ```
    pub fn from_notation(notation: &str) -> Board {
        let mut board = Board::new();
        for c in notation.chars() {
            board.play(Column::from(c));
        }
        board
    }

    /// Checks if a given column is playable, i.e. if there is still space in the column
    #[inline]
    pub fn is_playable(&self, column: Column) -> bool {
        self.height[column as usize] < HEIGHT
    }

    /// Checks if playing a piece in the given column would result in a win by the current player
    pub fn is_winning(&self, column: Column) -> bool {
        if !self.is_playable(column) {
            return false;
        }

        let column = column as usize;
        let row = self.height[column];
        let player = self.get_current_player();

        if row >= 3 {
            // Check vertical
            if self.slots[row - 1][column] == player &&
               self.slots[row - 2][column] == player &&
               self.slots[row - 3][column] == player {
                return true;
            }
        }

        // Check horizontal and diagonals for number of adjacent pieces
        for dy in -1..=1 {  // dy = -1 is \, dy = 0 is -, dy = 1 is /
            let mut n_adjacent = 0;
            for dx in -1..=1 {
                if dx == 0 {
                    continue;
                }

                let mut x = column as i32 + dx;
                let mut y = row as i32 + dx*dy;
                while x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                    if self.slots[y as usize][x as usize] == player {
                        n_adjacent += 1;
                    } else {
                        break;
                    }

                    x += dx;
                    y += dy*dx;
                }

                if n_adjacent >= 3 {
                    return true;
                }
            }
        }

        false
    }

    /// Plays a piece in the given column, if possible
    /// Returns the number of played moves.
    /// One can compare to the previous number of moves to check if the move was valid
    /// A winning move is considered not valid for this function. Before playing, one should check whether the move is winning
    /// by calling [Board::is_winning()]
    pub fn play(&mut self, column: Column) -> u32 {
        if !self.is_playable(column) || self.is_winning(column) {
            return self.n_moves;
        }

        let column = column as usize;
        let row = self.height[column];
        let player = self.get_current_player();

        self.slots[row][column] = player;
        self.height[column] += 1;
        self.n_moves += 1;

        self.n_moves
    }

    /// Returns the number of moves made so far
    #[inline]
    pub fn number_of_moves(&self) -> u32 {
        self.n_moves
    }

    #[inline]
    fn get_current_player(&self) -> Slot {
        let player = self.n_moves % 2;
        if player == 0 {
            Slot::P1
        } else {
            Slot::P2
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(HEIGHT * (WIDTH + 1));
        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                let c = match self.slots[row][col] {
                    Slot::Empty => '-',
                    Slot::P1 => 'X',
                    Slot::P2 => 'O'
                };
                s.push(c);
            }
            s.push('\n');
        }
        write!(f, "{}", s)?;
        let next_player = self.get_current_player();
        let next_player = if next_player == Slot::P1 { "X" } else { "O" };
        writeln!(f, "Next player: {}", next_player)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_is_playable() {
        let mut board = Board::new();
        Column::iter().for_each(|c| assert!(board.is_playable(c)));

        for _ in 0..HEIGHT {
            board.play(Column::A);
        }

        assert!(!board.is_playable(Column::A));
    }

    #[test]
    fn test_is_winning_horizontal() {
        let board = Board::from_notation("435462");
        assert!(board.is_winning(Column::G));
        Column::iter().filter(|&c| c != Column::G).for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_is_winning_vertical() {
        let board = Board::from_notation("123451121517");
        assert!(board.is_winning(Column::A));
        Column::iter().skip(1).for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_is_winning_diagonal() {
        let board = Board::from_notation("453433222");
        assert!(board.is_winning(Column::B));

        let board = Board::from_notation("2334454551");
        assert!(board.is_winning(Column::E));
    }

    #[test]
    fn test_is_winning_2_places() {
        let board = Board::from_notation("445362322111");
        assert!(board.is_winning(Column::A));  // diagonal win
        assert!(board.is_winning(Column::G));  // horizontal win

        Column::iter().filter(|&c| c != Column::G && c != Column::A)
        .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn winning_move_doesnt_count() {
        let mut board = Board::from_notation("123451121517");  // Wins with A
        assert!(board.is_winning(Column::A));

        let current_moves = board.number_of_moves();
        assert_eq!(board.play(Column::A), current_moves);  // number doesn't change
    }

    #[test]
    fn test_play() {
        let mut board = Board::new();
        assert_eq!(board.play(Column::D), 1);
        assert_eq!(board.play(Column::E), 2);
        assert_eq!(board.play(Column::D), 3);
        assert_eq!(board.play(Column::G), 4);
    }
}
