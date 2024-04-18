use lazy_static::lazy_static;

use crate::{
    board::{self, ArrayBoard, Player},
    N,
};

#[inline(always)]
fn index_from_ijk(i: usize, j: usize, k: usize) -> usize {
    k * N * N + i * N + j
}

#[inline(always)]
fn index_from_index_2d(index_2d: usize, k: usize) -> usize {
    k * N * N + index_2d
}

#[inline(always)]
fn index_to_ijk(index: usize) -> (usize, usize, usize) {
    (index / N % N, index % N, index / N / N)
}

#[inline(always)]
fn index_to_bit(index: usize) -> u64 {
    1u64 << (index)
}

lazy_static! {
    static ref CHECK_MASK_TABLE: Vec<Vec<u64>> = {
        let mut vec = vec![];

        for index in 0..N * N * N {
            let (i, j, k) = index_to_ijk(index);
            let mut masks = vec![];

            // 4 in z
            if k == N - 1 {
                masks.push((0..N-1).map(|kp| index_from_ijk(i, j, kp)).map(|b| index_to_bit(b)).sum());
            }

            // i
            masks.push(
                (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, j, k)).map(|b| index_to_bit(b)).sum()
            );

            // j
            masks.push(
                (0..N).filter(|&jp| jp != j).map(|jp| index_from_ijk(i, jp, k)).map(|b| index_to_bit(b)).sum()
            );

            if i == j {
                masks.push(
                    (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, ip, k)).map(|b| index_to_bit(b)).sum()
                );
            }

            if i == N - 1 - j {
                masks.push(
                    (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, N - 1 - ip, k)).map(|b| index_to_bit(b)).sum()
                );
            }

            if i == k {
                masks.push(
                    (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, j, ip)).map(|b| index_to_bit(b)).sum()
                );
            }

            if i == N - 1 - k {
                masks.push(
                    (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, j, N - 1 - ip)).map(|b| index_to_bit(b)).sum()
                );
            }

            if j == k {
                masks.push(
                    (0..N).filter(|&jp| jp != j).map(|jp| index_from_ijk(i, jp, jp)).map(|b| index_to_bit(b)).sum()
                );
            }

            if j == N - 1 - k {
                masks.push(
                    (0..N).filter(|&jp| jp != j).map(|jp| index_from_ijk(i, jp, N - 1 - jp)).map(|b| index_to_bit(b)).sum()
                );
            }

            // i - j - k
            if i == j && i == k {
                masks.push(
                    (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, ip, ip)).map(|b| index_to_bit(b)).sum()
                );
            }

            if i == N - 1 - j && i == k {
                masks.push(
                    (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, N - 1 - ip, ip)).map(|b| index_to_bit(b)).sum()
                );
            }

            if i == j && i == N - 1 - k {
                masks.push(
                    (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, ip, N - 1 - ip)).map(|b| index_to_bit(b)).sum()
                );
            }

            if i == N - 1 - j && i == N - 1 - k {
                masks.push(
                    (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, N - 1 - ip, N - 1 - ip)).map(|b| index_to_bit(b)).sum()
                );
            }

            vec.push(masks);
        }
        vec
    };
}

const BLACK_INDEX: usize = 0;
const WHITE_INDEX: usize = 1;

fn player_index(player: Player) -> usize {
    match player {
        Player::Black => BLACK_INDEX,
        Player::White => WHITE_INDEX,
    }
}

#[derive(Clone, Debug)]
pub struct BitBoard {
    pub(crate) boards: [u64; 2],
    pub(crate) next_player: Player,
}

impl Into<ArrayBoard> for &BitBoard {
    fn into(self) -> ArrayBoard {
        let mut result = ArrayBoard::new();
        result.next_player = self.next_player;
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    let index = index_from_ijk(i, j, k);
                    result.board[i * N + j][k] = if self.boards[player_index(Player::Black)]
                        & index_to_bit(index)
                        != 0
                    {
                        board::Piece::Black
                    } else if self.boards[player_index(Player::White)] & index_to_bit(index) != 0 {
                        board::Piece::White
                    } else {
                        board::Piece::Empty
                    }
                }
            }
        }
        result
    }
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard {
            boards: [0u64, 0u64],
            next_player: Player::Black,
        }
    }

    pub fn show(&self) {
        for i in 0..N {
            for k in 0..N {
                for j in 0..N {
                    let index = index_from_ijk(i, j, k);

                    if self.boards[player_index(Player::Black)] & index_to_bit(index) != 0 {
                        print!("b");
                    } else if self.boards[player_index(Player::White)] & index_to_bit(index) != 0 {
                        print!("w");
                    } else {
                        print!("e");
                    }
                }
                print!("\t")
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn find_index(&self, index_2d: usize) -> Option<usize> {
        let board: u64 = self.boards.iter().sum();
        (0..N)
            .map(|k| index_from_index_2d(index_2d, k))
            .find(|index| board & index_to_bit(*index) == 0)
    }

    fn put_without_check(&self, index: usize) -> Self {
        let mut new_board = self.clone();
        new_board.boards[player_index(self.next_player)] |= index_to_bit(index);
        new_board.next_player = self.next_player.next_player();
        new_board
    }

    pub fn put(&self, index_2d: usize) -> Option<Self> {
        let index = self.find_index(index_2d)?;
        Some(self.put_without_check(index))
    }

    fn simple_policy(&self, index: usize) -> usize {
        CHECK_MASK_TABLE[index]
            .iter()
            .map(|&mask| {
                let my_piece = self.boards[player_index(self.next_player)] & mask;
                let opp_piece = self.boards[player_index(self.next_player.next_player())] & mask;

                if (my_piece == 0 && opp_piece != 0) || (my_piece != 0 && opp_piece == 0) {
                    2
                } else if my_piece == 0 && opp_piece == 0 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn count_policy(&self, index: usize) -> usize {
        // スコアは適当
        let boards: u64 = self.boards.iter().sum();
        CHECK_MASK_TABLE[index]
            .iter()
            .map(|&mask| {
                let my_piece = self.boards[player_index(self.next_player)] & mask;
                let opp_piece = self.boards[player_index(self.next_player.next_player())] & mask;

                if (my_piece == 0 && opp_piece != 0) || (my_piece != 0 && opp_piece == 0) {
                    let ones = my_piece.count_ones() + opp_piece.count_ones();
                    if ones == 1 {
                        2
                    } else {
                        // 2石が揃っている。3石目。
                        let last_one = ((!my_piece) & mask) & ((!opp_piece) & mask);
                        // 下の段が埋まっているか。
                        if boards & (last_one >> N * N) != 0 {
                            // ただのリーチ
                            1
                        } else {
                            // トラップ
                            if last_one & 0xFFFF << 2 * N * N != 0 {
                                // 三段目トラップ
                                8
                            } else {
                                // 二or四段目トラップ
                                5
                            }
                        }
                    }
                } else if my_piece == 0 && opp_piece == 0 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    /**
     * 今の打ち手の評価関数の値も返す。
     */
    pub fn put_with_simple_policy(&self, index_2d: usize) -> Option<(Self, usize)> {
        let index = self.find_index(index_2d)?;
        Some((self.put_without_check(index), self.simple_policy(index)))
    }

    pub fn put_with_count_policy(&self, index_2d: usize) -> Option<(Self, usize)> {
        let index = self.find_index(index_2d)?;
        Some((self.put_without_check(index), self.count_policy(index)))
    }

    pub fn is_full(&self) -> bool {
        (!self.boards.iter().sum::<u64>()) == 0u64
    }

    fn win_index_2d_player(&self, player: Player) -> Option<usize> {
        for i in 0..N {
            for j in 0..N {
                let index_2d = i * N + j;
                if let Some(index) = self.find_index(index_2d) {
                    if CHECK_MASK_TABLE[index]
                        .iter()
                        .any(|mask| self.boards[player_index(player)] & mask == *mask)
                    {
                        return Some(index_2d);
                    }
                }
            }
        }
        None
    }

    /**
     * return true if the current player wins
     */
    pub fn win_index(&self) -> Option<usize> {
        self.win_index_2d_player(self.next_player)
    }

    /**
     * return true if the current player is checked
     */
    pub fn check_index(&self) -> Option<usize> {
        self.win_index_2d_player(self.next_player.next_player())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::ArrayBoard;

    use super::*;

    #[test]
    fn test_ab() {
        for _ in 0..10000 {
            let mut arrayboard = ArrayBoard::new();
            let mut bitboard = BitBoard::new();
            loop {
                assert_eq!(arrayboard.is_full(), bitboard.is_full());

                if arrayboard.is_full() {
                    break;
                }

                let index = (rand::random::<usize>()) % (N * N);
                let bt = bitboard.put(index);
                let at = arrayboard.put(index);

                assert_eq!(at.is_none(), bt.is_none());

                if at.is_none() {
                    continue;
                }

                arrayboard = at.unwrap();
                bitboard = bt.unwrap();

                assert_eq!(arrayboard.check_index(), bitboard.check_index());
                assert_eq!(arrayboard.win_index(), bitboard.win_index());

                if arrayboard.win_index().is_some() {
                    break;
                }
            }
        }
    }
}
