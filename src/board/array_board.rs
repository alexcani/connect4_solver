use crate::board::{Board, Column, HEIGHT, WIDTH};
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
enum Slot {
    #[default]
    Empty,
    P1,
    P2,
}

/// A Connect 4 board represented by bidimensional array of slots
///
/// To create a new board, check [ArrayBoard::new()] and [ArrayBoard::from_notation()]
///
/// To check how to use a board to play, check [Board]
#[derive(Copy, Clone, Debug, Default)]
pub struct ArrayBoard {
    slots: [[Slot; WIDTH]; HEIGHT],
    height: [usize; WIDTH], // Number of pieces in each column
    n_moves: u32,
}

impl ArrayBoard {
    /// Creates a new, empty board
    pub fn new() -> ArrayBoard {
        ArrayBoard {
            slots: [[Slot::Empty; WIDTH]; HEIGHT],
            height: [0; WIDTH],
            n_moves: 0,
        }
    }

    /// Create a new board from a given position, using the sequence of played columns notation
    ///
    /// Example:
    /// ```
    /// # use connect4_solver::board::ArrayBoard;
    /// let board = ArrayBoard::from_notation("32164625");
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
    pub fn from_notation(notation: &str) -> ArrayBoard {
        let mut board = ArrayBoard::new();
        for c in notation.chars() {
            board.play(Column::from(c));
        }
        board
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

impl Board for ArrayBoard {
    #[inline]
    fn is_playable(&self, column: Column) -> bool {
        self.height[column as usize] < HEIGHT
    }

    fn is_winning(&self, column: Column) -> bool {
        if !self.is_playable(column) {
            return false;
        }

        let column = column as usize;
        let row = self.height[column];
        let player = self.get_current_player();

        if row >= 3 {
            // Check vertical
            if self.slots[row - 1][column] == player
                && self.slots[row - 2][column] == player
                && self.slots[row - 3][column] == player
            {
                return true;
            }
        }

        // Check horizontal and diagonals for number of adjacent pieces
        for dy in -1..=1 {
            // dy = -1 is \, dy = 0 is -, dy = 1 is /
            let mut n_adjacent = 0;
            for dx in -1..=1 {
                if dx == 0 {
                    continue;
                }

                let mut x = column as i32 + dx;
                let mut y = row as i32 + dx * dy;
                while x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                    if self.slots[y as usize][x as usize] == player {
                        n_adjacent += 1;
                    } else {
                        break;
                    }

                    x += dx;
                    y += dy * dx;
                }

                if n_adjacent >= 3 {
                    return true;
                }
            }
        }

        false
    }

    fn play(&mut self, column: Column) -> u32 {
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

    #[inline]
    fn number_of_moves(&self) -> u32 {
        self.n_moves
    }
}

impl Display for ArrayBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(HEIGHT * (WIDTH + 1));
        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                let c = match self.slots[row][col] {
                    Slot::Empty => '-',
                    Slot::P1 => 'X',
                    Slot::P2 => 'O',
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
        let mut board = ArrayBoard::new();
        Column::iter().for_each(|c| assert!(board.is_playable(c)));

        for _ in 0..HEIGHT {
            board.play(Column::A);
        }

        assert!(!board.is_playable(Column::A));
    }

    #[test]
    fn test_is_winning_horizontal() {
        let board = ArrayBoard::from_notation("435462");
        assert!(board.is_winning(Column::G));
        Column::iter()
            .filter(|&c| c != Column::G)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_is_winning_vertical() {
        let board = ArrayBoard::from_notation("123451121517");
        assert!(board.is_winning(Column::A));
        Column::iter()
            .skip(1)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_is_winning_diagonal() {
        let board = ArrayBoard::from_notation("453433222");
        assert!(board.is_winning(Column::B));

        let board = ArrayBoard::from_notation("2334454551");
        assert!(board.is_winning(Column::E));
    }

    #[test]
    fn test_is_winning_2_places() {
        let board = ArrayBoard::from_notation("445362322111");
        assert!(board.is_winning(Column::A)); // diagonal win
        assert!(board.is_winning(Column::G)); // horizontal win

        Column::iter()
            .filter(|&c| c != Column::G && c != Column::A)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn winning_move_doesnt_count() {
        let mut board = ArrayBoard::from_notation("123451121517"); // Wins with A
        assert!(board.is_winning(Column::A));

        let current_moves = board.number_of_moves();
        assert_eq!(board.play(Column::A), current_moves); // number doesn't change
    }

    #[test]
    fn test_play() {
        let mut board = ArrayBoard::new();
        assert_eq!(board.play(Column::D), 1);
        assert_eq!(board.play(Column::E), 2);
        assert_eq!(board.play(Column::D), 3);
        assert_eq!(board.play(Column::G), 4);
    }
}
