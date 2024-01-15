use std::fmt::Display;

use crate::board::{Board, Column, HEIGHT, WIDTH};
use static_assertions as sa;

type BitBoardField = u64;

#[derive(Copy, Clone, Debug, Default)]
pub struct BitBoard {
    n_moves: usize,
    // height stores the bit index of the first empty slot in each column, not the number of pieces
    height: [usize; WIDTH],
    pos: [BitBoardField; 2],
}

sa::const_assert!(std::mem::size_of::<BitBoardField>() <= (HEIGHT + 1) * WIDTH);

impl Board for BitBoard {
    fn is_playable(&self, column: Column) -> bool {
        let top = 1 << (HEIGHT - 1) << (column as usize * (HEIGHT + 1));
        (self.pos[0] | self.pos[1]) & top == 0
    }

    fn is_winning(&self, column: Column) -> bool {
        if !self.is_playable(column) {
            return false;
        }

        let pos = self.pos[self.n_moves % 2] | 1 << self.height[column as usize];

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

        self.pos[self.n_moves % 2] |= 1 << self.height[column as usize];

        self.height[column as usize] += 1;
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
            height,
            pos: [0; 2],
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
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in (0..HEIGHT).rev() {
            for column in 0..WIDTH {
                let pos = 1 << (row + column * (HEIGHT + 1));
                if self.pos[0] & pos != 0 {
                    s.push('X');
                } else if self.pos[1] & pos != 0 {
                    s.push('O');
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
        println!("{}", board);
        assert!(board.is_winning(Column::G));
        Column::iter()
            .filter(|&c| c != Column::G)
            .for_each(|c| assert!(!board.is_winning(c)));
    }

    #[test]
    fn test_is_winning_vertical() {
        let board = BitBoard::from_notation("123451121517");
        println!("{:?}", board);
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
