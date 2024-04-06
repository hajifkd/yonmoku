pub const N: usize = 4;

pub mod board;
pub mod mctree;

pub fn unpack_index(index: usize) -> (usize, usize) {
    (index / N, index % N)
}
