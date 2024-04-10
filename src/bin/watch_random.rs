use rand::random;
use yonmoku::{board::Board, mctree_old::McTreeRoot, unpack_index, N};

fn next(board: Board, stone: usize) -> Option<(usize, f32)> {
    let n_try = 50_000;
    let mut tree = McTreeRoot::new(board);
    tree.select(n_try * (1 + stone * stone / 24))
}

/**
 * ランダム選択。ただし王手には必ず応手する。
 */
fn random_choose(board: &Board) -> Option<usize> {
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

fn main() {
    let mut board = Board::new();
    let mut stone = 0;
    loop {
        stone += 1;
        if let Some(hand) = random_choose(&board) {
            println!("{}: CPU2: {:?}", stone, unpack_index(hand));
            board = board.put(hand).unwrap();
            board.show();
        } else {
            println!("draw");
            return;
        }

        if let Some(index) = board.win_index() {
            println!("CPU 1 win, put {:?}", unpack_index(index));
            return;
        }

        stone += 1;
        if let Some((hand, rate)) = next(board.clone(), stone) {
            println!(
                "{}: CPU1: {:?}, Rate for Sente: {:.1}%",
                stone,
                unpack_index(hand),
                100f32 - rate * 100f32
            );
            board = board.put(hand).unwrap();
            board.show();
        } else {
            println!("draw");
            return;
        }

        if let Some(index) = board.win_index() {
            println!("CPU 2 win, put {:?}", unpack_index(index));
            return;
        }
    }
}
