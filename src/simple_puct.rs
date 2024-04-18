use std::marker::PhantomData;

use crate::{bitboard::BitBoard, N};
use rand::random;
use rayon::prelude::*;

const N_TRIAL_THRESHOLD: usize = 20;

#[derive(Debug)]
pub struct McTreeRoot<T: Policy> {
    current_board: BitBoard,
    leaves: [Option<McTreeLeaf<T>>; N * N],
    policy_type: PhantomData<fn() -> T>,
}

#[derive(Debug)]
struct McTreeLeaf<T: Policy> {
    current_board: BitBoard,
    n_trial: usize,
    n_win: usize,
    n_lose: usize,
    policy: usize,
    leaves: Option<Vec<McTreeLeaf<T>>>,
    is_checked: bool,
    policy_type: PhantomData<fn() -> T>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum McResult {
    Win,
    Lose,
    Draw,
}

pub trait Policy {
    fn put_with_policy(board: &BitBoard, index_2d: usize) -> Option<(BitBoard, usize)>;
}

pub struct SimplePolicy;
pub struct CountPolicy;

impl Policy for SimplePolicy {
    fn put_with_policy(board: &BitBoard, index_2d: usize) -> Option<(BitBoard, usize)> {
        board.put_with_simple_policy(index_2d)
    }
}

impl Policy for CountPolicy {
    fn put_with_policy(board: &BitBoard, index_2d: usize) -> Option<(BitBoard, usize)> {
        board.put_with_count_policy(index_2d)
    }
}

impl<T: Policy> Clone for McTreeLeaf<T> {
    fn clone(&self) -> Self {
        Self {
            current_board: self.current_board.clone(),
            n_trial: self.n_trial.clone(),
            n_win: self.n_win.clone(),
            n_lose: self.n_lose.clone(),
            policy: self.policy.clone(),
            leaves: self.leaves.clone(),
            is_checked: self.is_checked.clone(),
            policy_type: PhantomData,
        }
    }
}

impl<T: Policy> McTreeLeaf<T> {
    pub fn new(board: BitBoard, policy: usize) -> Self {
        McTreeLeaf {
            is_checked: board.check_index().is_some(),
            current_board: board,
            n_trial: 0,
            n_win: 0,
            n_lose: 0,
            policy: policy,
            leaves: None,
            policy_type: PhantomData,
        }
    }

    fn win_rate(&self) -> f32 {
        (self.n_win as f32 + ((self.n_trial - self.n_win - self.n_lose) as f32) / 2.0)
            / (self.n_trial as f32)
    }

    pub fn select_rate(&self, n_try: usize) -> f32 {
        let c = 0.2f32;
        (1f32 - self.win_rate())
            + c * (self.policy as f32) * ((n_try as f32).sqrt() / self.n_trial as f32)
    }

    fn run(&mut self) -> McResult {
        self.n_trial += 1;
        let mut board = self.current_board.clone();

        while !board.is_full() {
            if board.win_index().is_some() {
                // 次打つプレイヤーが勝利する
                if board.next_player == self.current_board.next_player {
                    self.n_win += 1;
                    return McResult::Win;
                } else {
                    self.n_lose += 1;
                    return McResult::Lose;
                }
            }

            // 王手がかかっていたら、解除する
            if let Some(index) = board.check_index() {
                board = board.put(index).unwrap();
                continue;
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

    fn run_and_push(&mut self, index: usize) -> Option<McResult> {
        if let Some((board, policy)) = T::put_with_policy(&self.current_board, index) {
            self.n_trial += 1;
            let mut leaf = McTreeLeaf::new(board, policy);
            let result = leaf.run();
            if result == McResult::Win {
                self.n_lose += 1;
            } else {
                self.n_win += 1;
            }
            self.leaves.as_mut().unwrap().push(leaf);
            Some(result)
        } else {
            None
        }
    }

    // return (try, win, lose)
    pub fn expand(&mut self) -> (usize, usize, usize) {
        if self.current_board.win_index().is_some() {
            self.n_trial += 1;
            self.n_win += 1;

            return (1, 1, 0);
        }

        self.leaves = Some(vec![]);

        if self.is_checked {
            let index = self.current_board.check_index().unwrap();
            let result = self.run_and_push(index).unwrap();
            return match result {
                McResult::Win => (1, 1, 0),
                McResult::Lose => (1, 0, 1),
                McResult::Draw => (1, 0, 0),
            };
        }

        let mut n_trial = 0;
        let mut n_win = 0;
        let mut n_lose = 0;

        for i in 0..N {
            for j in 0..N {
                let index = i * N + j;
                if let Some(result) = self.run_and_push(index) {
                    n_trial += 1;
                    match result {
                        McResult::Win => n_win += 1,
                        McResult::Lose => n_lose += 1,
                        McResult::Draw => (),
                    }
                }
            }
        }

        (n_trial, n_win, n_lose)
    }

    // return (try, win, lose)
    pub fn select(&mut self) -> (usize, usize, usize) {
        if let Some(leaves) = &mut self.leaves {
            // choose appropriate k
            let k = leaves
                .into_iter()
                .map(|o| (o.select_rate(self.n_trial), o))
                .max_by(|(r1, _), (r2, _)| r1.partial_cmp(r2).unwrap_or(std::cmp::Ordering::Equal));
            if let Some((_, o)) = k {
                let (n_trial, n_win, n_lose) = o.select();
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
        } else if self.is_checked || self.n_trial > N_TRIAL_THRESHOLD {
            // 王手がかかっていたら試行回数は無視する。
            self.expand()
        } else {
            match self.run() {
                McResult::Win => (1, 1, 0),
                McResult::Lose => (1, 0, 1),
                McResult::Draw => (1, 0, 0),
            }
        }
    }
}

impl<T: Policy> McTreeRoot<T> {
    pub fn new(board: BitBoard) -> Self {
        let leaves = (0..N * N)
            .map(|index| T::put_with_policy(&board, index).map(|(b, p)| McTreeLeaf::<T>::new(b, p)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("never reach here."));
        McTreeRoot {
            current_board: board,
            leaves: leaves,
            policy_type: PhantomData,
        }
    }

    /**
     * return (hand, eval)
     */
    pub fn select(&mut self, n_total: usize) -> Option<(usize, f32)> {
        if let Some(index) = self.current_board.check_index() {
            return Some((index, -100f32));
        }

        (0..N * N)
            .par_bridge()
            .filter_map(|index| {
                let mut leaf = (self.leaves[index]).clone();
                if leaf.is_none() {
                    return None;
                }
                for _ in 0..n_total {
                    leaf.as_mut().unwrap().select();
                }
                Some((leaf.unwrap().win_rate(), index))
            })
            .max_by(|(k1, _), (k2, _)| k2.partial_cmp(k1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(rate, index)| (index, rate))
    }
}
