use yonmoku::{board::Board, mctree, mctree_old};

fn next_mcts_ucb1_old(board: Board, stone: usize) -> Option<usize> {
    let n_try = 5_000;
    let mut tree = mctree_old::McTreeRoot::new(board);
    tree.select(n_try * (1 + stone * stone / 16))
        .map(|(hand, _)| hand)
}

fn next_mcts_ucb1(board: Board, stone: usize) -> Option<usize> {
    let n_try = 5_000;
    let mut tree = mctree::McTreeRoot::new(board);
    tree.select(n_try * (1 + stone * stone / 16))
        .map(|(hand, _)| hand)
}

fn battle(
    n_try: usize,
    cpu_sente: fn(Board, usize) -> Option<usize>,
    cpu_gote: fn(Board, usize) -> Option<usize>,
) -> (f64, f64) {
    let mut n_win = 0;
    let mut n_draw = 0;
    for _i in 0..n_try {
        let mut board = Board::new();
        let mut stone = 0;

        'game: loop {
            if let Some(hand) = cpu_sente(board.clone(), stone) {
                stone += 1;
                board = board.put(hand).unwrap();
            } else {
                n_draw += 1;
                break 'game;
            }

            if board.win_index().is_some() {
                break 'game;
            }

            if let Some(hand) = cpu_gote(board.clone(), stone) {
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
    let (sente_win, sente_draw) = battle(10, next_mcts_ucb1, next_mcts_ucb1_old);
    println!(
        "New MCTS UCB1 sente win_rate: {:.4}, draw_rate: {:.4}",
        sente_win, sente_draw
    );

    let (gote_win, gote_draw) = battle(10, next_mcts_ucb1_old, next_mcts_ucb1);
    println!(
        "New MCTS UCB1 gote win_rate: {:.4}, draw_rate: {:.4}",
        gote_win, gote_draw
    );
}
