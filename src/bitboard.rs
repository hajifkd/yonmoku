use lazy_static::lazy_static;

use crate::{board::Player, N};

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
struct Piece(u8);

const BLACK_PIECE: Piece = Piece(0b01);
const WHITE_PIECE: Piece = Piece(0b10);
const ANY_PIECE: Piece = Piece(0b11);
const EMPTY_PIECE: Piece = Piece(0b00);

impl Into<Piece> for Player {
    fn into(self) -> Piece {
        match self {
            Player::Black => BLACK_PIECE,
            Player::White => WHITE_PIECE,
        }
    }
}

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
fn index_to_bit(piece: u128, index: usize) -> u128 {
    piece << (2 * index)
}

lazy_static! {
    static ref CHECK_MASK_TABLE: [Vec<Vec<u128>>; 2] = {
        let mut result = [vec![], vec![]];

        for piece in (BLACK_PIECE.0 as usize)..=(WHITE_PIECE.0 as usize) {
            let vec = &mut result[piece - 1];

            for index in 0..N * N * N {
                let (i, j, k) = index_to_ijk(index);
                let mut masks = vec![];

                // 4 in z
                if k == N - 1 {
                    masks.push((0..N-1).map(|kp| index_from_ijk(i, j, kp)).map(|b| index_to_bit(piece as _, b)).sum());
                }

                // i
                masks.push(
                    (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, j, k)).map(|b| index_to_bit(piece as _, b)).sum()
                );

                // j
                masks.push(
                    (0..N).filter(|&jp| jp != j).map(|jp| index_from_ijk(i, jp, k)).map(|b| index_to_bit(piece as _, b)).sum()
                );

                if i == j {
                    masks.push(
                        (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, ip, k)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                if i == N - 1 - j {
                    masks.push(
                        (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, N - 1 - ip, k)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                if i == k {
                    masks.push(
                        (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, j, ip)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                if i == N - 1 - k {
                    masks.push(
                        (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, j, N - 1 - ip)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                if j == k {
                    masks.push(
                        (0..N).filter(|&jp| jp != j).map(|jp| index_from_ijk(i, jp, jp)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                if j == N - 1 - k {
                    masks.push(
                        (0..N).filter(|&jp| jp != j).map(|jp| index_from_ijk(i, jp, N - 1 - jp)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                // i - j - k
                if i == j && i == k {
                    masks.push(
                        (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, ip, ip)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                if i == N - 1 - j && i == k {
                    masks.push(
                        (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, N - 1 - ip, ip)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                if i == j && i == N - 1 - k {
                    masks.push(
                        (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, ip, N - 1 - ip)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                if i == N - 1 - j && i == N - 1 - k {
                    masks.push(
                        (0..N).filter(|&ip| ip != i).map(|ip| index_from_ijk(ip, N - 1 - ip, N - 1 - ip)).map(|b| index_to_bit(piece as _, b)).sum()
                    );
                }

                vec.push(masks);
            }
        }
        result
    };
}

#[derive(Clone, Debug)]
pub struct BitBoard {
    board: u128,
    pub(crate) next_player: Player,
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard {
            board: 0u128,
            next_player: Player::Black,
        }
    }

    pub fn show(&self) {
        for i in 0..N {
            for k in 0..N {
                for j in 0..N {
                    let index = index_from_ijk(i, j, k);
                    match Piece(((self.board >> (index * 2)) as u8) & ANY_PIECE.0) {
                        EMPTY_PIECE => print!("e"),
                        BLACK_PIECE => print!("b"),
                        WHITE_PIECE => print!("w"),
                        _ => print!("?"),
                    }
                }
                print!("\t")
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn find_index(&self, index_2d: usize) -> Option<usize> {
        (0..N)
            .map(|k| index_from_index_2d(index_2d, k))
            .find(|index| self.board & index_to_bit(ANY_PIECE.0 as _, *index) == 0)
    }

    pub fn put(&self, index_2d: usize) -> Option<Self> {
        let index = self.find_index(index_2d)?;
        let new_board = {
            let mut new_board = self.clone();
            new_board.board |=
                index_to_bit(Into::<Piece>::into(new_board.next_player).0 as _, index);
            new_board.next_player = self.next_player.next_player();
            new_board
        };
        Some(new_board)
    }

    pub fn is_full(&self) -> bool {
        (0..N * N * N)
            .all(|index| ((self.board >> (index * 2)) as u8) & ANY_PIECE.0 != EMPTY_PIECE.0)
    }

    fn win_index_2d_player(&self, player: Player) -> Option<usize> {
        let piece = {
            let p: Piece = player.into();
            p.0 as usize
        };
        for i in 0..N {
            for j in 0..N {
                let index_2d = i * N + j;
                if let Some(index) = self.find_index(index_2d) {
                    if CHECK_MASK_TABLE[piece - 1][index]
                        .iter()
                        .any(|mask| self.board & mask == *mask)
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
