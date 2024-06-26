use yonmoku::{
    bitboard::BitBoard,
    mctree,
    simple_puct::{self, CountPolicy, Policy, SimplePolicy},
};

fn next_mcts_ucb1(board: BitBoard, stone: usize) -> Option<usize> {
    let n_try = 50_000;
    let mut tree = mctree::McTreeRoot::new(board);
    tree.select(n_try * (1 + stone * stone / 24))
        .map(|(hand, _)| hand)
}

fn next_mcts_puct<T: Policy>(board: BitBoard, stone: usize) -> Option<usize> {
    let n_try = 50_000;
    let mut tree = simple_puct::McTreeRoot::<T>::new(board);
    tree.select(n_try * (1 + stone * stone / 24))
        .map(|(hand, _)| hand)
}

fn battle(
    n_try: usize,
    cpu_sente: fn(BitBoard, usize) -> Option<usize>,
    cpu_gote: fn(BitBoard, usize) -> Option<usize>,
) -> (f64, f64) {
    let mut n_win = 0;
    let mut n_draw = 0;
    for _i in 0..n_try {
        let mut board = BitBoard::new();
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
    let (sente_win, sente_draw) = battle(
        100,
        next_mcts_puct::<CountPolicy>,
        next_mcts_puct::<SimplePolicy>,
    );
    println!(
        "MCTS PUCT CountPolicy sente win_rate: {:.4}, draw_rate: {:.4}",
        sente_win, sente_draw
    );

    let (gote_lose, gote_draw) = battle(
        100,
        next_mcts_puct::<SimplePolicy>,
        next_mcts_puct::<CountPolicy>,
    );
    println!(
        "MCTS PUCT CountPolicy gote win_rate: {:.4}, draw_rate: {:.4}",
        1f64 - gote_lose,
        gote_draw
    );
}
