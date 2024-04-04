use crate::{board::Board, N};
use rand::random;
use rayon::prelude::*;

#[derive(Debug)]
pub struct McTreeRoot {
    leaves: [Option<McTreeLeaf>; N * N],
}

#[derive(Debug, Clone)]
pub struct McTreeLeaf {
    current_board: Board,
    n_trial: usize,
    n_win: usize,
    n_lose: usize,
    leaves: Option<[Option<Box<McTreeLeaf>>; N * N]>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum McResult {
    Win,
    Lose,
    Draw,
}

impl McTreeLeaf {
    pub fn new(board: Board) -> Self {
        McTreeLeaf {
            current_board: board,
            n_trial: 0,
            n_win: 0,
            n_lose: 0,
            leaves: None,
        }
    }

    fn win_rate(&self) -> f32 {
        (self.n_win as f32 + ((self.n_trial - self.n_win - self.n_lose) as f32) / 2.0)
            / (self.n_trial as f32)
    }

    pub fn select_rate(&self, n_try: usize) -> f32 {
        let c = 1.4f32;
        (1f32 - self.win_rate()) + c * ((n_try as f32).ln() / self.n_trial as f32).sqrt()
    }

    fn run(&mut self) -> McResult {
        self.n_trial += 1;
        let mut board = self.current_board.clone();

        while !board.is_full() {
            if board.is_finished() {
                // 次打つプレイヤーが勝利する
                if board.next_player == self.current_board.next_player {
                    self.n_win += 1;
                    return McResult::Win;
                } else {
                    self.n_lose += 1;
                    return McResult::Lose;
                }
            }

            loop {
                if let Some(b) = {
                    let index = random::<usize>() & (N * N - 1);
                    board.put(index)
                } {
                    board = b;
                    break;
                }
            }
        }

        return McResult::Draw;
    }

    pub fn expand(&mut self) -> (usize, usize, usize) {
        let mut n_trial = 0;
        let mut n_win = 0;
        let mut n_lose = 0;
        let mut leaves: [Option<Box<McTreeLeaf>>; N * N] = Default::default();

        for i in 0..N {
            for j in 0..N {
                let index = i * N + j;
                if let Some(board) = self.current_board.put(index) {
                    n_trial += 1;
                    let mut leaf = Box::new(McTreeLeaf::new(board));
                    let result = leaf.run();
                    if result == McResult::Win {
                        n_lose += 1;
                    } else {
                        n_win += 1;
                    }
                    leaves[index] = Some(leaf);
                }
            }
        }

        self.leaves = Some(leaves);
        self.n_trial += n_trial;
        self.n_win += n_win;
        self.n_lose += n_lose;

        (n_trial, n_win, n_lose)
    }

    pub fn select(&mut self, n_try: usize) -> (usize, usize, usize) {
        if let Some(leaves) = &mut self.leaves {
            // choose appropriate k
            let k = leaves
                .into_iter()
                .filter(|o| o.is_some())
                .map(|o| (o.as_ref().map(|p| p.select_rate(n_try)).unwrap(), o))
                .max_by(|(r1, _), (r2, _)| r1.partial_cmp(r2).unwrap_or(std::cmp::Ordering::Equal));
            if let Some((_, o)) = k {
                let (n_trial, n_win, n_lose) = o.as_mut().unwrap().select(n_try);
                // flip win/lose and add
                self.n_trial += n_trial;
                self.n_win += n_lose;
                self.n_lose += n_win;
                (n_trial, n_lose, n_win)
            } else {
                // draw
                self.n_trial += 1;
                (1, 0, 0)
            }
        } else {
            self.expand()
        }
    }
}

impl McTreeRoot {
    pub fn new(board: Board) -> Self {
        let leaves = (0..N * N)
            .map(|index| board.put(index).map(McTreeLeaf::new))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        McTreeRoot { leaves: leaves }
    }

    pub fn select(&mut self, n_total: usize) -> Option<usize> {
        (0..N * N)
            .par_bridge()
            .filter_map(|index| {
                let mut leaf = (self.leaves[index]).clone();
                if leaf.is_none() {
                    return None;
                }
                for n in 0..n_total {
                    leaf.as_mut().unwrap().select(n + 1);
                }
                println!(
                    "player: {:?} index: {} select rate:{}",
                    leaf.as_ref().unwrap().current_board.next_player,
                    index,
                    leaf.as_ref().unwrap().win_rate()
                );
                Some((leaf.unwrap().win_rate(), index))
            })
            .max_by(|(k1, _), (k2, _)| k2.partial_cmp(k1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(_, index)| index)
    }
}
