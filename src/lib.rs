pub const N: usize = 4;

pub mod bitboard;
pub mod board;
pub mod mctree;
pub mod mctree_old;
pub mod simple_puct;

pub fn unpack_index(index: usize) -> (usize, usize) {
    (index / N, index % N)
}
