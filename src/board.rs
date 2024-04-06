use crate::N;

/**
 * State for board piece
 * Black for the first player and White for the second player.
 */
#[repr(u8)]
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Piece {
    #[default]
    Empty,
    Black,
    White,
}

/**
 * Player enum
 */
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    Black,
    White,
}

impl Into<Piece> for Player {
    fn into(self) -> Piece {
        match self {
            Player::Black => Piece::Black,
            Player::White => Piece::White,
        }
    }
}

impl Player {
    pub fn next_player(self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    board: [[Piece; N]; N * N],
    pub(crate) next_player: Player,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: Default::default(),
            next_player: Player::Black,
        }
    }

    pub fn show(&self) {
        for i in 0..N {
            for z in 0..N {
                for j in 0..N {
                    let index = i * N + j;
                    match self.board[index][z] {
                        Piece::Empty => print!("e"),
                        Piece::Black => print!("b"),
                        Piece::White => print!("w"),
                    }
                }
                print!("\t")
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn find_index(&self, index: usize) -> Option<usize> {
        self.board[index]
            .iter()
            .enumerate()
            .find(|(_, &p)| p == Piece::Empty)
            .map(|(k, _)| k)
    }

    pub fn put(&self, index: usize) -> Option<Self> {
        let k = self.find_index(index)?;
        let new_board = {
            let mut new_board = self.clone();
            new_board.board[index][k] = new_board.next_player.into();
            new_board.next_player = self.next_player.next_player();
            new_board
        };
        Some(new_board)
    }

    pub fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|ps| ps.into_iter().all(|&p| p != Piece::Empty))
    }

    fn win_index_player(&self, player: Player) -> Option<usize> {
        let piece = player.into();
        for i in 0..N {
            for j in 0..N {
                let index = i * N + j;
                if let Some(k) = self.find_index(index) {
                    // 4 in z
                    if k == N - 1 && self.board[index].iter().take(N - 1).all(|&p| p == piece) {
                        return Some(index);
                    }
                    // 4 in x
                    if (0..N).all(|ip| i == ip || self.board[ip * N + j][k] == piece) {
                        return Some(index);
                    }
                    // 4 in y
                    if (0..N).all(|jp| j == jp || self.board[i * N + jp][k] == piece) {
                        return Some(index);
                    }
                    // 4 in x-y
                    if i == j && (0..N).all(|ip| i == ip || self.board[ip * N + ip][k] == piece) {
                        return Some(index);
                    }
                    // 4 in y-x
                    if i + j == N - 1
                        && (0..N).all(|ip| i == ip || self.board[ip * N + (N - 1 - ip)][k] == piece)
                    {
                        return Some(index);
                    }
                    // 4 in y-z
                    if i == k && (0..N).all(|ip| i == ip || self.board[ip * N + j][ip] == piece) {
                        return Some(index);
                    }
                    // 4 in z-y
                    if i + k == N - 1
                        && (0..N).all(|ip| i == ip || self.board[ip * N + j][N - 1 - ip] == piece)
                    {
                        return Some(index);
                    }
                    // 4 in x-z
                    if j == k && (0..N).all(|jp| j == jp || self.board[i * N + jp][jp] == piece) {
                        return Some(index);
                    }
                    // 4 in z-x
                    if j + k == N - 1
                        && (0..N).all(|jp| j == jp || self.board[i * N + jp][N - 1 - jp] == piece)
                    {
                        return Some(index);
                    }
                    // 4 in x-y-z
                    if i == j
                        && i == k
                        && (0..N).all(|ip| i == ip || self.board[ip * N + ip][ip] == piece)
                    {
                        return Some(index);
                    }
                    // 4 in y-x-z
                    if i + j == N - 1
                        && j == k
                        && (0..N).all(|ip| {
                            i == ip || self.board[ip * N + (N - 1 - ip)][N - 1 - ip] == piece
                        })
                    {
                        return Some(index);
                    }
                    // 4 in z-y-x
                    if i == j
                        && i + k == N - 1
                        && (0..N).all(|ip| i == ip || self.board[ip * N + ip][N - 1 - ip] == piece)
                    {
                        return Some(index);
                    }
                    // 4 in z-x-y
                    if i + j == N - 1
                        && i == k
                        && (0..N)
                            .all(|ip| i == ip || self.board[ip * N + (N - 1 - ip)][ip] == piece)
                    {
                        return Some(index);
                    }
                } else {
                    continue;
                }
            }
        }

        None
    }

    /**
     * return true if the current player wins
     */
    pub fn win_index(&self) -> Option<usize> {
        self.win_index_player(self.next_player)
    }

    /**
     * return true if the current player is checked
     */
    pub fn check_index(&self) -> Option<usize> {
        self.win_index_player(self.next_player.next_player())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win() {
        let mut board = Board::new();
        assert_eq!(board.win_index().is_some(), false);
        for k in 0..N * N {
            board = Board::new();
            for i in 0..N - 1 {
                board.board[k][i] = Piece::Black;
            }
            assert_eq!(board.win_index(), Some(k));
        }
        board = Board::new();
        board.board[0][0] = Piece::Black;
        board.board[5][0] = Piece::White;
        board.board[10][0] = Piece::White;
        board.board[10][1] = Piece::White;
        board.board[10][2] = Piece::Black;
        board.board[15][0] = Piece::White;
        board.board[15][1] = Piece::White;
        board.board[15][2] = Piece::White;
        board.board[15][3] = Piece::Black;
        assert_eq!(board.win_index(), Some(5));

        board = Board::new();
        board.board[3][0] = Piece::Black;
        board.board[6][0] = Piece::White;
        board.board[9][0] = Piece::White;
        board.board[9][1] = Piece::White;
        board.board[9][2] = Piece::Black;
        board.board[12][0] = Piece::White;
        board.board[12][1] = Piece::White;
        board.board[12][2] = Piece::White;
        board.board[12][3] = Piece::Black;
        assert_eq!(board.win_index(), Some(6));
    }
}
