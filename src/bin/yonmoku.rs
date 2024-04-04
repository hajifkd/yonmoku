use std::io;

use yonmoku::{board::Board, mctree::McTreeRoot, N};

fn next(board: Board) -> Option<usize> {
    let n_try = 200_000;
    let mut tree = McTreeRoot::new(board);
    tree.select(n_try)
}

fn main() -> io::Result<()> {
    let mut sente = true;
    while {
        println!("[S]ente or [G]ote?");
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer)?;
        let buffer = buffer.trim_end();
        match buffer {
            "S" => false,
            "G" => {
                sente = false;
                false
            }
            _ => true,
        }
    } {}

    let mut board = Board::new();

    if !sente {
        let hand = next(board.clone()).unwrap();
        println!("CPU: {},{}", hand / N, hand % N);
        board.show();
        board = board.put(hand).unwrap();
        board.show();
    }

    loop {
        let mut human = (0, 0);
        while {
            println!("i,j?");
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer)?;
            let buffer = buffer.trim_end();
            let input = buffer.split(",").collect::<Vec<_>>();

            if input.len() != 2 {
                true
            } else {
                if let (Ok(i), Ok(j)) = (input[0].parse::<usize>(), input[1].parse::<usize>()) {
                    if N > i && N > j {
                        human = (i, j);
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
        } {}
        if let Some(board_human) = board.put(human.0 * N + human.1) {
            board = board_human;
            board.show();
            if board.is_finished() {
                println!("You lose");
                break;
            }
            let hand = next(board.clone());
            if hand.is_none() {
                break;
            }
            let hand = hand.unwrap();
            //println!("{:?}", board.find_index(0));
            println!("CPU: {},{}", hand / N, hand % N);
            board = board.put(hand).unwrap();
            board.show();
            if board.is_finished() {
                println!("You win");
                break;
            }
        }
    }
    Ok(())
}
