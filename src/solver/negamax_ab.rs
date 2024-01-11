use crate::solver::score;
use crate::board::*;
use strum::IntoEnumIterator;

// Naive implementation of negamax without any optimizations
pub fn solve(position: &Board, nodes_searched: &mut usize) -> i32 {
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
            best = best.max(-solve(&next_position, nodes_searched));
        }
    }

    best
}
