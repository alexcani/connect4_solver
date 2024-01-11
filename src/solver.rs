use crate::board::*;

use strum::IntoEnumIterator;

/// The result of a solve operation, containing the score of the position for the current player
/// and the number of searched nodes.
pub struct SolveResult {
    pub score: i32,
    pub nodes_searched: usize,
}

/// Solves a given position by using the negamax variant of the minmax algorithm,
/// returning the position's score for the current player and the number of searched nodes.
pub fn negamax(position: &Board) -> SolveResult {
    let mut nodes_searched = 0;
    let score = negamax_impl(position, &mut nodes_searched);
    SolveResult { score, nodes_searched }
}

fn negamax_impl(position: &Board, nodes_searched: &mut usize) -> i32 {
    *nodes_searched += 1;

    // Stop conditions
    // 1 - Draw. All moves have been made without a win
    if position.number_of_moves() == WIDTH as u32 * HEIGHT as u32 {
        return 0;
    }

    // 2 - Win for current player
    for column in Column::iter() {
        if position.is_winning(column) {
            return score(position.number_of_moves());
        }
    };

    let mut best = -((WIDTH*HEIGHT) as i32);
    for column in Column::iter() {
        if position.is_playable(column) {
            let mut next_position = *position;
            next_position.play(column);
            best = best.max(-negamax_impl(&next_position, nodes_searched));
        }
    }

    best
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
