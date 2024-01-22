use crate::board::*;
use crate::solver::{score, SolveResult};
use crate::transposition_table::TranspositionTable;
use strum::{EnumCount, IntoEnumIterator};

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

#[derive(Default)]
pub struct NegamaxSolver {
    table: Option<TranspositionTable>,
}

impl NegamaxSolver {
    pub fn new() -> Self {
        Self { table: None }
    }

    pub fn new_with_table() -> Self {
        Self {
            table: Some(TranspositionTable::default()),
        }
    }

    pub fn new_with_custom_table(table: TranspositionTable) -> Self {
        Self { table: Some(table) }
    }

    pub fn solve(&mut self, position: &impl Board) -> SolveResult {
        let mut nodes_searched = 0;
        let ab = (WIDTH*HEIGHT) as i32 / 2;
        let score = self.solve_impl(position, &mut nodes_searched, -ab, ab);
        SolveResult { score, nodes_searched }
    }

    fn solve_impl(
        &mut self,
        position: &impl Board,
        nodes_searched: &mut usize,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        *nodes_searched += 1;

        // Stop conditions
        // 1 - Draw. All moves have been made without a win
        if position.number_of_moves() == WIDTH as u32 * HEIGHT as u32 {
            return 0;
        }

        // 2 - Win for current player
        for column in Column::iter() {
            if position.is_playable(column) && position.is_winning(column) {
                return score(position.number_of_moves());
            }
        }

        // Maximum achievable score since position.number_of_moves() moves have been made so far
        // This maximum score changes every turn, so we need to account of it in beta before iterating
        let mut max = ((WIDTH * HEIGHT - 1) as u32 - position.number_of_moves()) as i32 / 2;

        // Check transposition table
        const MIN_SCORE: i32 = -((WIDTH * HEIGHT) as i32 / 2) + 3;
        if position.has_key() && self.table.is_some() {
            if let Some(score) = self.table.as_ref().unwrap().get(position.key()) {
                max = score as i32 + MIN_SCORE - 1;
            }
        }

        if beta > max {  // the lower bound of the position score is the best the opponent can do (new upper bound for us)
            beta = max;
            if alpha >= beta {
                return alpha;
            }
        }

        for column in COLUMN_ORDER {
            if position.is_playable(column) {
                let mut next_position = *position;
                next_position.play(column);
                let score = -self.solve_impl(
                    &next_position,
                    nodes_searched,
                    -beta,
                    -alpha,
                );
                if score >= beta {
                    // our possible score is better than the worst score the opponent can make us get
                    return score;
                }
                alpha = alpha.max(score);
            }
        }

        if position.has_key() && self.table.is_some() {
            self.table.as_mut()
                .unwrap()
                .set(position.key(), (alpha - MIN_SCORE + 1) as u8);
        }
        alpha
    }
}
