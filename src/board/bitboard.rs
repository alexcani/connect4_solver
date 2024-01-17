use std::fmt::Display;

use crate::board::{Board, Column, HEIGHT, WIDTH};
use static_assertions as sa;

type BitBoardField = u64;

#[derive(Copy, Clone, Debug, Default)]
pub struct BitBoard {
    n_moves: usize,
    pos: BitBoardField,  // stores the positions of the pieces of the current player
    mask: BitBoardField,  // marks all non-empty cells
}

sa::const_assert!(std::mem::size_of::<BitBoardField>() <= (HEIGHT + 1) * WIDTH);

impl Board for BitBoard {
    #[inline]
    fn is_playable(&self, column: Column) -> bool {
        self.mask & BitBoard::top_mask(column) == 0
    }

    fn is_winning(&self, column: Column) -> bool {
        // Play the stone in the position (mask + bottom mask), select only the column played (result is a single 1 bit somewhere).
        // Then add it to the position of the current player
        let pos = self.pos | ((self.mask + BitBoard::bottom_mask(column)) & BitBoard::column_mask(column));

        // Vertical
        let m = pos & (pos >> 1);
        if m & (m >> 2) != 0 {
            return true;
        }

        // Horizontal
        let m = pos & (pos >> (HEIGHT + 1));
        if m & (m >> (2 * (HEIGHT + 1))) != 0 {
            return true;
        }

        // Diagonal \
        let m = pos & (pos >> HEIGHT);
        if m & (m >> (2 * HEIGHT)) != 0 {
            return true;
        }

        // Diagonal /
        let m = pos & (pos >> (HEIGHT + 2));
        if m & (m >> (2 * (HEIGHT + 2))) != 0 {
            return true;
        }

        false
    }

    #[inline]
    fn number_of_moves(&self) -> u32 {
        self.n_moves as u32
    }

    fn play(&mut self, column: Column) -> u32 {
        if self.is_winning(column) || !self.is_playable(column) {
            return self.n_moves as u32;
        }

        self.pos ^= self.mask;  // switch player
        self.mask |= self.mask + BitBoard::bottom_mask(column);  // play in the column

        self.n_moves += 1;
        self.n_moves as u32
    }
}

impl BitBoard {
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
    fn bottom_mask(column: Column) -> BitBoardField {
        1 << (column as usize * (HEIGHT + 1))
    }

    #[inline]
    fn column_mask(column: Column) -> BitBoardField {
        ((1 << HEIGHT) - 1) << (column as usize * (HEIGHT + 1))
    }

    #[inline]
    fn top_mask(column: Column) -> BitBoardField {
        1 << (HEIGHT - 1) << (column as usize * (HEIGHT + 1))
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
        Column::iter()
            .filter(|&c| c != Column::G)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_is_winning_vertical() {
        let board = BitBoard::from_notation("123451121517");
        assert!(board.is_winning(Column::A));
        Column::iter()
            .skip(1)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_is_winning_diagonal() {
        let board = BitBoard::from_notation("453433222");
        assert!(board.is_winning(Column::B));

        let board = BitBoard::from_notation("2334454551");
        assert!(board.is_winning(Column::E));
    }

    #[test]
    fn test_is_winning_2_places() {
        let board = BitBoard::from_notation("445362322111");
        assert!(board.is_winning(Column::A)); // diagonal win
        assert!(board.is_winning(Column::G)); // horizontal win

        Column::iter()
            .filter(|&c| c != Column::G && c != Column::A)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn winning_move_doesnt_count() {
        let mut board = BitBoard::from_notation("123451121517"); // Wins with A
        assert!(board.is_winning(Column::A));

        let current_moves = board.number_of_moves();
        assert_eq!(board.play(Column::A), current_moves); // number doesn't change
    }

    #[test]
    fn test_play() {
        let mut board = BitBoard::new();
        assert_eq!(board.play(Column::D), 1);
        assert_eq!(board.play(Column::E), 2);
        assert_eq!(board.play(Column::D), 3);
        assert_eq!(board.play(Column::G), 4);
    }
}
