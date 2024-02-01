//! This module contains the board trait and all board implementations

use static_assertions as sa;
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter, FromRepr};

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

#[derive(Copy, Clone, PartialEq, Debug, EnumIter, FromRepr, EnumCount)]
pub enum Column {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
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
            _ => panic!("Invalid column"),
        }
    }
}

/// A Connect 4 board that can be played on or passed into a solver
pub trait Board: Copy {
    /// Checks if a given column is playable, i.e. if there is still space in the column
    fn is_playable(&self, column: Column) -> bool;

    /// Checks if playing a piece in the given column would result in a win by the current player
    fn is_winning(&self, column: Column) -> bool;

    /// Plays a piece in the given column
    /// Returns the number of played moves.
    /// Before playing, one should check whether the move is winning by calling [Board::is_winning()]
    /// This method should not be called if the move is not playable or winning
    fn play(&mut self, column: Column) -> u32;

    /// Returns the number of moves made so far
    fn number_of_moves(&self) -> u32;

    /// Returns the unique key that represented the position.
    fn key(&self) -> u64;

    /// Returns an array of the possible non-losing moves. A true value at index i means that column i can be played
    /// A move is non-losing if it doesn't result in an immediate win for the opponent
    /// Thing function should not be called if there is a move that immediately wins the game
    /// for the current player. To check that, use [Board::can_win_in_one_move()]
    fn possible_nonlosing_moves(&self) -> [bool; WIDTH];

    /// Returns whether the current player can win in the next move
    fn can_win_in_one_move(&self) -> bool;
}

// Implementation of a Bitboard

type BitBoardField = u64;

#[derive(Copy, Clone, Debug, Default)]
pub struct BitBoard {
    n_moves: usize,
    pos: BitBoardField, // stores the positions of the pieces of the current player
    mask: BitBoardField, // marks all non-empty cells
}

sa::const_assert!(std::mem::size_of::<BitBoardField>() <= (HEIGHT + 1) * WIDTH);

impl Board for BitBoard {
    #[inline]
    fn is_playable(&self, column: Column) -> bool {
        self.mask & BitBoard::top_mask_col(column) == 0
    }

    fn is_winning(&self, column: Column) -> bool {
        self.possible_moves() & self.winning_position() & BitBoard::column_mask(column) != 0
    }

    #[inline]
    fn number_of_moves(&self) -> u32 {
        self.n_moves as u32
    }

    fn play(&mut self, column: Column) -> u32 {
        self.pos ^= self.mask; // switch player
        self.mask |= self.mask + BitBoard::bottom_mask_col(column); // play in the column

        self.n_moves += 1;
        self.n_moves as u32
    }

    #[inline]
    fn key(&self) -> u64 {
        self.pos + self.mask
    }

    fn can_win_in_one_move(&self) -> bool {
        self.possible_moves() & self.winning_position() != 0
    }

    fn possible_nonlosing_moves(&self) -> [bool; WIDTH] {
        assert!(!self.can_win_in_one_move(),
        "Called possible_nonlosing_moves but there is a move that immediately wins the game for the current player");

        let mut possible = self.possible_moves();
        let opponent_win = self.opponent_winning_position();
        let forced_moves = possible & opponent_win;
        if forced_moves != 0 {
            if forced_moves & (forced_moves - 1) != 0 {
                // more than one forced move, we can't do anything
                return [false; WIDTH];
            }

            possible = forced_moves;
        }

        // Don't play directly under an opponent's winning position as well
        possible &= !(opponent_win >> 1);
        if possible == 0 {
            return [false; WIDTH];
        }

        let mut moves = [false; WIDTH];
        for (i, column) in Column::iter().enumerate() {
            if possible & BitBoard::column_mask(column) != 0 {
                moves[i] = true;
            }
        }

        moves
    }
}

impl BitBoard {
    // 1 on the bottom row of each column
    const BOTTOM_MASK: BitBoardField = BitBoard::bottom(WIDTH, HEIGHT);
    // 1 on every cell of the board
    const BOARD_MASK: BitBoardField = BitBoard::BOTTOM_MASK * ((1 << HEIGHT) - 1);

    pub fn new() -> Self {
        let mut height = [0; WIDTH];
        for (i, item) in height.iter_mut().enumerate() {
            *item = i * (HEIGHT + 1);
        }

        BitBoard {
            n_moves: 0,
            pos: 0,
            mask: 0,
        }
    }

    pub fn from_notation(notation: &str) -> Self {
        let mut board = BitBoard::new();
        for c in notation.chars() {
            let column = Column::from(c);
            board.play(column);
        }
        board
    }

    #[inline]
    fn bottom_mask_col(column: Column) -> BitBoardField {
        1 << (column as usize * (HEIGHT + 1))
    }

    #[inline]
    fn column_mask(column: Column) -> BitBoardField {
        ((1 << HEIGHT) - 1) << (column as usize * (HEIGHT + 1))
    }

    #[inline]
    fn top_mask_col(column: Column) -> BitBoardField {
        1 << (HEIGHT - 1) << (column as usize * (HEIGHT + 1))
    }

    #[inline]
    fn possible_moves(&self) -> BitBoardField {
        (self.mask + BitBoard::BOTTOM_MASK) & BitBoard::BOARD_MASK
    }

    // Returns a bitmask of the possible winning moves for the current player
    fn winning_position(&self) -> BitBoardField {
        BitBoard::compute_winning_position(self.pos, self.mask)
    }

    // Returns a bitmask of the possible winning moves for the opponent
    fn opponent_winning_position(&self) -> BitBoardField {
        BitBoard::compute_winning_position(self.pos ^ self.mask, self.mask)
    }

    // Recursively construct a bitmask with 1 on the bottom row of each column
    const fn bottom(width: usize, height: usize) -> BitBoardField {
        if width == 0 {
            0
        } else {
            1 << ((height + 1) * (width - 1)) | BitBoard::bottom(width - 1, height)
        }
    }

    // Returns a bitmask of the possible winning moves for the current position (player) and mask
    const fn compute_winning_position(
        position: BitBoardField,
        mask: BitBoardField,
    ) -> BitBoardField {
        let mut moves = 0;

        // Resulting bitmask is the actual move, because of the shifts
        let vertical = (position << 1) & (position << 2) & (position << 3);
        moves |= vertical;

        let horizontal = (position << (HEIGHT + 1)) & (position << (2 * (HEIGHT + 1)));
        moves |= horizontal & (position << (3 * (HEIGHT + 1))); // horizontally to the left
        moves |= horizontal & (position >> (HEIGHT + 1)); // horizontally to the right

        let horizontal = (position >> (HEIGHT + 1)) & (position >> (2 * (HEIGHT + 1)));
        moves |= horizontal & (position >> (3 * (HEIGHT + 1))); // horizontally to the right
        moves |= horizontal & (position << (HEIGHT + 1)); // horizontally to the left

        // Diagonal 1
        let diag = (position << HEIGHT) & (position << (2 * HEIGHT));
        moves |= diag & (position << (3 * HEIGHT)); // diagonally to the left
        moves |= diag & (position >> HEIGHT); // diagonally to the right

        let diag = (position >> HEIGHT) & (position >> (2 * HEIGHT));
        moves |= diag & (position >> (3 * HEIGHT)); // diagonally to the right
        moves |= diag & (position << HEIGHT); // diagonally to the left

        // Diagonal 2
        let diag = (position << (HEIGHT + 2)) & (position << (2 * (HEIGHT + 2)));
        moves |= diag & (position << (3 * (HEIGHT + 2))); // diagonally to the left
        moves |= diag & (position >> (HEIGHT + 2)); // diagonally to the right

        let diag = (position >> (HEIGHT + 2)) & (position >> (2 * (HEIGHT + 2)));
        moves |= diag & (position >> (3 * (HEIGHT + 2))); // diagonally to the right
        moves |= diag & (position << (HEIGHT + 2)); // diagonally to the left

        moves & (mask ^ BitBoard::BOARD_MASK)
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in (0..HEIGHT).rev() {
            for column in 0..WIDTH {
                let pos = 1 << (row + column * (HEIGHT + 1));
                let is_stone = self.mask & pos != 0;
                let is_stone_current_player = self.pos & pos != 0;
                let is_p1 = self.n_moves % 2 == 0;

                if is_stone {
                    if is_stone_current_player {
                        s.push(if is_p1 { 'X' } else { 'O' });
                    } else {
                        s.push(if is_p1 { 'O' } else { 'X' });
                    }
                } else {
                    s.push('-');
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_is_playable() {
        let mut board = BitBoard::new();
        Column::iter().for_each(|c| assert!(board.is_playable(c)));

        for _ in 0..HEIGHT {
            board.play(Column::A);
        }

        assert!(!board.is_playable(Column::A));
    }

    #[test]
    fn test_is_winning_horizontal() {
        let board = BitBoard::from_notation("435462");
        assert!(board.is_winning(Column::G));
        assert!(board.can_win_in_one_move());
        Column::iter()
            .filter(|&c| c != Column::G)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_is_winning_vertical() {
        let board = BitBoard::from_notation("123451121517");
        assert!(board.is_winning(Column::A));
        assert!(board.can_win_in_one_move());
        Column::iter()
            .skip(1)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_is_winning_diagonal() {
        let board = BitBoard::from_notation("453433222");
        assert!(board.is_winning(Column::B));
        assert!(board.can_win_in_one_move());

        let board = BitBoard::from_notation("2334454551");
        assert!(board.is_winning(Column::E));
        assert!(board.can_win_in_one_move());
    }

    #[test]
    fn test_is_winning_2_places() {
        let board = BitBoard::from_notation("445362322111");
        assert!(board.is_winning(Column::A)); // diagonal win
        assert!(board.is_winning(Column::G)); // horizontal win
        assert!(board.can_win_in_one_move());

        Column::iter()
            .filter(|&c| c != Column::G && c != Column::A)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_play() {
        let mut board = BitBoard::new();
        assert_eq!(board.play(Column::D), 1);
        assert_eq!(board.play(Column::E), 2);
        assert_eq!(board.play(Column::D), 3);
        assert_eq!(board.play(Column::G), 4);
    }

    #[test]
    fn test_possible_nonlosing_moves() {
        let board = BitBoard::new();
        let moves = board.possible_nonlosing_moves();
        assert_eq!(moves, [true; WIDTH]); // all columns are possible

        // Losing position
        let board = BitBoard::from_notation("4453623221115");

        // Player 2's turn, but player 1 can win in A or G. So there's nothing player 2 can do
        assert_eq!(board.possible_nonlosing_moves(), [false; WIDTH]); // all columns are false

        // Player 1 can win in E
        let mut board = BitBoard::from_notation("2334465545");
        assert!(board.is_winning(Column::E));
        board.play(Column::A); // don't win yet
        assert_eq!(board.possible_nonlosing_moves(), [false, false, false, false, true, false, false]); // only E is possible otherwise p1 wins
    }
}
