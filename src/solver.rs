mod negamax;

use crate::board::*;

/// The result of a solve operation, containing the score of the position for the current player
/// and the number of searched nodes.
pub struct SolveResult {
    pub score: i32,
    pub nodes_searched: usize,
}

/// Solves a position by using the negamax variant of the minmax algorithm,
/// returning the position's score for the current player and the number of searched nodes.
pub fn negamax(position: &Board) -> SolveResult {
    let mut nodes_searched = 0;
    let score = negamax::solve(position, &mut nodes_searched);
    SolveResult { score, nodes_searched }
}

#[inline]
fn score(n_moves: u32) -> i32 {
    ((WIDTH*HEIGHT + 1) as i32 - n_moves as i32) / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score() {
        // Win on 4th stone of player 1 -> each player played 3 so far
        assert_eq!(score(6), 18);
        // 4th stone of player 2 -> P1 played 4, P2 played 3
        assert_eq!(score(7), 18);

        // 18th stone of player 1 -> P1 played 17, P2 played 17
        assert_eq!(score(34), 4);
        // 18th stone of player 2 -> P1 played 18, P2 played 17
        assert_eq!(score(35), 4);
    }
}
