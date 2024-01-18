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
    const MID: i32 = (Column::COUNT / 2) as i32;
    let mut order = [Column::A; WIDTH];
    let mut index: i32 = 0;
    loop {
        order[index as usize] = unwrap_col(Column::from_repr((MID - ((1-2*(index%2))*(index+1)/2)) as usize));
        index += 1;
        if index >= WIDTH as i32 {
            break;
        }
    }

    order
}

pub fn solve(position: &impl Board, nodes_searched: &mut usize, mut alpha: i32, mut beta: i32) -> i32 {
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
