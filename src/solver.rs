//! This module contains functions and structs to solve a Connect 4 position.
use crate::board::*;
use crate::transposition_table::TranspositionTable;
use strum::EnumCount;
use std::collections::BinaryHeap;

// Generate move order based on constant WIDTH instead of hardcoding it
const COLUMN_ORDER: [Column; WIDTH] = generate_move_order();
const fn unwrap_col(c: Option<Column>) -> Column {
    match c {
        Some(c) => c,
        None => panic!("Invalid column"),
    }
}
const fn generate_move_order() -> [Column; WIDTH] {
    const MID: i32 = (Column::COUNT / 2) as i32;
    let mut order = [Column::A; WIDTH];
    let mut index: i32 = 0;
    loop {
        order[index as usize] = unwrap_col(Column::from_repr(
            (MID - ((1 - 2 * (index % 2)) * (index + 1) / 2)) as usize,
        ));
        index += 1;
        if index >= WIDTH as i32 {
            break;
        }
    }

    order
}

/// The result of a solve operation, containing the score of the position for the current player
/// and the number of searched nodes.
pub struct SolveResult {
    pub score: i32,
    pub nodes_searched: usize,
}

#[derive(Default)]
pub struct Solver {
    table: TranspositionTable,
}

// Public API
impl Solver {
    pub fn new() -> Self {
        Self {
            table: TranspositionTable::default(),
        }
    }

    pub fn new_with_table(table: TranspositionTable) -> Self {
        Self { table }
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }

    pub fn solve(&mut self, position: &impl Board) -> SolveResult {
        if position.can_win_in_one_move() {
            return SolveResult {
                score: score(position.number_of_moves()),
                nodes_searched: 1,
            };
        }

        let mut min = -(WIDTH as i32 * HEIGHT as i32 - position.number_of_moves() as i32) / 2;
        let mut max = (WIDTH as i32 * HEIGHT as i32 + 1 - position.number_of_moves() as i32) / 2;
        let mut nodes = 0;

        while min < max {
            let mut mid = min + (max - min) / 2;
            if mid <= 0 && min / 2 < mid {
                mid = min / 2;
            } else if mid >= 0 && max / 2 > mid {
                mid = max / 2;
            }

            // Since the score is bounded by the number of moves, there's an implicit depth limit in the search that
            // depends on beta.
            let mut nodes_searched = 0;
            let score = self.solve_impl(position, &mut nodes_searched, mid, mid + 1);
            if score > mid {
                min = score;
            } else {
                max = score;
            }
            nodes += nodes_searched;
        }

        SolveResult {
            score: min,
            nodes_searched: nodes,
        }
    }
}

// Private API
impl Solver {
    fn solve_impl(
        &mut self,
        position: &impl Board,
        nodes_searched: &mut usize,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        *nodes_searched += 1;

        let possible_moves = position.possible_nonlosing_moves();

        // Stop conditions
        // 1 - No possible non-losing moves -> opponent wins next turn
        if possible_moves.is_none() {
            return -((WIDTH * HEIGHT) as i32 - position.number_of_moves() as i32) / 2;
        }

        // 2 - Draw. All moves have been made without a win (actually, prune a bit ealier since a win is no longer possible at this point)
        if position.number_of_moves() >= (WIDTH as u32 * HEIGHT as u32) - 2 {
            return 0;
        }

        // Lower bound since opponent cannot win next move (possible moves are not empty)
        let min = -((WIDTH * HEIGHT - 2) as i32 - position.number_of_moves() as i32) / 2;
        if alpha < min {
            // update alpha and possibly prune
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }

        // Maximum achievable score since position.number_of_moves() moves have been made so far
        // This maximum score changes every turn, so we need to account of it in beta before iterating
        let mut max = ((WIDTH * HEIGHT - 1) as u32 - position.number_of_moves()) as i32 / 2;

        // Check transposition table
        const MIN_SCORE: i32 = -((WIDTH * HEIGHT) as i32 / 2) + 3;
        if let Some(score) = self.table.get(position.key()) {
            max = score as i32 + MIN_SCORE - 1;
        }

        if beta > max {
            // the lower bound of the position score is the best the opponent can do (new upper bound for us)
            beta = max;
            if alpha >= beta {
                return alpha;
            }
        }

        let possible_moves = possible_moves.unwrap();

        // Sort moves by priority, defaulting to priority in COLUMN_ORDER
        let mut heap = BinaryHeap::with_capacity(possible_moves.len());
        for column in COLUMN_ORDER {
            if possible_moves[column as usize] {
                heap.push(position.score_move(column));
            }
        }

        while let Some(ScoredMove { column, .. }) = heap.pop() {
            let mut next_position = *position;
            next_position.play(column);
            let score = -self.solve_impl(&next_position, nodes_searched, -beta, -alpha);
            if score >= beta {
                // our possible score is better than the worst score the opponent can make us get
                return score;
            }
            alpha = alpha.max(score);
        }

        self.table
            .set(position.key(), (alpha - MIN_SCORE + 1) as u8);

        alpha
    }
}

#[inline]
fn score(n_moves: u32) -> i32 {
    ((WIDTH * HEIGHT + 1) as i32 - n_moves as i32) / 2
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
