use crate::solver::score;
use crate::board::*;
use strum::{IntoEnumIterator, EnumCount};

// Generate move order based on constant WIDTH instead of hardcoding it
const COLUMN_ORDER: [Column; WIDTH] = generate_move_order();
const fn unwrap_col(c: Option<Column>) -> Column {
    match c {
        Some(c) => c,
        None => panic!("Invalid column"),
    }
}
const fn generate_move_order() -> [Column; WIDTH] {
    let mut order = [Column::A; WIDTH];
    let mid = Column::COUNT / 2;
    let mut inc = 1;
    let mut index = 1;
    order[0] = unwrap_col(Column::from_repr(mid));
    loop {
        order[index] = unwrap_col(Column::from_repr(mid - inc));
        if mid + inc < Column::COUNT {
            order[index+1] = unwrap_col(Column::from_repr(mid + inc));
        } else {
            break;
        }
        if mid - inc == 0 {
            break;
        }

        inc += 1;
        index += 2;
    }

    order
}

pub fn solve(position: &impl Board, nodes_searched: &mut usize, alpha: i32, beta: i32) -> i32 {
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
    };

    // Alpha-beta pruning
    let mut alpha = alpha;
    let mut beta = beta;

    // Maximum achievable score since position.number_of_moves() moves have been made so far
    // This maximum score changes every turn, so we need to account of it in beta before iterating
    let max = ((WIDTH*HEIGHT - 1) as u32 - position.number_of_moves()) as i32 / 2;
    if beta > max {
        beta = max;
        if alpha >= beta {
            return beta;
        }
    }

    for column in COLUMN_ORDER {
        if position.is_playable(column) {
            let mut next_position = *position;
            next_position.play(column);
            let score = -solve(&next_position, nodes_searched, -beta, -alpha);
            if score >= beta {  // our possible score is better than the worst score the opponent can make us get
                return score;
            }
            alpha = alpha.max(score);
        }
    }

    alpha
}
