use crate::board::*;
use crate::solver::SolveResult;
use crate::solver::NegamaxSolver;
use crate::transposition_table::TranspositionTable;

#[derive(Default)]
pub struct NegamaxID {
    solver: NegamaxSolver,
}

impl NegamaxID {
    pub fn new() -> Self {
        Self {
            solver: NegamaxSolver::new_with_table(),
        }
    }

    pub fn new_with_custom_table(table: TranspositionTable) -> Self {
        Self {
            solver: NegamaxSolver::new_with_custom_table(table),
        }
    }

    pub fn solve(&mut self, position: &impl Board) -> SolveResult {
        let mut min = -(WIDTH as i32*HEIGHT as i32 - position.number_of_moves() as i32) / 2;
        let mut max = (WIDTH as i32*HEIGHT as i32 + 1 - position.number_of_moves() as i32) / 2;
        let mut nodes = 0;

        while min < max {
            let mut mid = min + (max - min) / 2;
            if mid <= 0 && min/2 < mid {
                mid = min/2;
            } else if mid >= 0 && max/2 > mid {
                mid = max/2;
            }

            // Since the score is bounded by the number of moves, there's an implicit depth limit in the search that
            // depends on beta.
            let SolveResult{score, nodes_searched} = self.solver.solve_ab(position, mid, mid + 1);
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
