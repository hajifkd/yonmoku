use rand::random;
use yonmoku::{board::ArrayBoard, mctree_old::McTreeRoot, N};

fn next_mcts_ucb1(board: ArrayBoard, stone: usize) -> Option<usize> {
    let n_try = 5_000;
    let mut tree = McTreeRoot::new(board);
    tree.select(n_try * (1 + stone * stone / 16))
        .map(|(hand, _)| hand)
}

/**
 * ランダム選択。ただし王手には必ず応手する。
 */
fn random_choose(board: &ArrayBoard) -> Option<usize> {
    if board.is_full() {
        None
    } else {
        if let Some(hand) = board.check_index() {
            return Some(hand);
        }
        let mut index = random::<usize>() & (N * N - 1);
        while board.find_index(index).is_none() {
            index = random::<usize>() & (N * N - 1);
        }
        Some(index)
    }
}

fn battle(
    cpu_sente: bool,
    n_try: usize,
    cpu: fn(ArrayBoard, usize) -> Option<usize>,
) -> (f64, f64) {
    let mut n_win = 0;
    let mut n_draw = 0;
    for _i in 0..n_try {
        let mut board = ArrayBoard::new();
        let mut stone = 0;
        if !cpu_sente {
            board = board.put(random_choose(&board).unwrap()).unwrap();
            stone += 1;
        }

        'game: loop {
            if let Some(hand) = cpu(board.clone(), stone) {
                stone += 1;
                board = board.put(hand).unwrap();
            } else {
                n_draw += 1;
                break 'game;
            }

            if board.win_index().is_some() {
                break 'game;
            }

            if let Some(hand) = random_choose(&board) {
                stone += 1;
                board = board.put(hand).unwrap();
            } else {
                n_draw += 1;
                break 'game;
            }

            if board.win_index().is_some() {
                n_win += 1;
                break 'game;
            }
        }
    }

    (n_win as f64 / n_try as f64, n_draw as f64 / n_try as f64)
}

fn battle_self(n_try: usize, cpu: fn(ArrayBoard, usize) -> Option<usize>) -> (f64, f64) {
    let mut n_win = 0;
    let mut n_draw = 0;
    for _i in 0..n_try {
        let mut board = ArrayBoard::new();
        let mut stone = 0;

        'game: loop {
            if let Some(hand) = cpu(board.clone(), stone) {
                stone += 1;
                board = board.put(hand).unwrap();
            } else {
                n_draw += 1;
                break 'game;
            }

            if board.win_index().is_some() {
                break 'game;
            }

            if let Some(hand) = cpu(board.clone(), stone) {
                stone += 1;
                board = board.put(hand).unwrap();
            } else {
                n_draw += 1;
                break 'game;
            }

            if board.win_index().is_some() {
                n_win += 1;
                break 'game;
            }
        }
    }

    (n_win as f64 / n_try as f64, n_draw as f64 / n_try as f64)
}

fn main() {
    /*let (sente_win, sente_draw) = battle(true, 10, next_mcts_ucb1);
    println!(
        "MCTS UCB1 sente win_rate: {:.4}, draw_rate: {:.4}",
        sente_win, sente_draw
    );

    let (gote_win, gote_draw) = battle(false, 10, next_mcts_ucb1);
    println!(
        "MCTS UCB1 gote win_rate: {:.4}, draw_rate: {:.4}",
        gote_win, gote_draw
    );*/

    let (sente_win, sente_draw) = battle_self(100, next_mcts_ucb1);
    println!(
        "MCTS UCB1 sente win_rate: {:.4}, draw_rate: {:.4}",
        sente_win, sente_draw
    );
}
