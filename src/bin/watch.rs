use yonmoku::{board::Board, mctree_old::McTreeRoot, unpack_index};

fn next(board: Board, stone: usize) -> Option<(usize, f32)> {
    let n_try = 50_000;
    let mut tree = McTreeRoot::new(board);
    tree.select(n_try * (1 + stone * stone / 24))
}

fn main() {
    let mut board = Board::new();
    let mut stone = 0;
    loop {
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

        stone += 1;
        if let Some((hand, rate)) = next(board.clone(), stone) {
            println!(
                "{}: CPU2: {:?}, Rate for Sente: {:.1}%",
                stone,
                unpack_index(hand),
                rate * 100f32
            );
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
    }
}
