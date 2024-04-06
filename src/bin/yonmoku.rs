use std::io;

use yonmoku::{board::Board, mctree::McTreeRoot, unpack_index, N};

fn next(board: Board, stone: usize) -> Option<(usize, f32)> {
    let n_try = 50_000;
    let mut tree = McTreeRoot::new(board);
    tree.select(n_try * (1 + stone * stone / 24))
}

fn prompt(msg: &str) -> io::Result<String> {
    println!("{}", msg);
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    let buffer = buffer.trim_end();
    Ok(buffer.to_owned())
}

fn main() -> io::Result<()> {
    let mut sente = true;
    while {
        let buffer: &str = &prompt("[S]ente or [G]ote?")?;
        match buffer {
            "S" => false,
            "G" => {
                sente = false;
                false
            }
            _ => true,
        }
    } {}

    let mut boards = vec![Board::new()];
    let mut stone = 0;

    if !sente {
        let board = boards.last().unwrap().clone();
        let (hand, rate) = next(board.clone(), stone).unwrap();
        println!(
            "CPU: {:?}, CPU Rate: {}%",
            unpack_index(hand),
            100 - (rate * 100f32) as i32
        );
        board.show();
        boards.push(board.put(hand).unwrap());
        boards.last().unwrap().show();
        stone += 1;
    }

    'game: loop {
        let mut human = (0, 0);
        let board = boards.last().unwrap().clone();
        while {
            let buffer = prompt("i,j?[M]atta?")?;
            let input = buffer.split(",").collect::<Vec<_>>();

            if buffer == "M" {
                if boards.len() > 2 {
                    boards.pop();
                    boards.pop();
                    boards.last().unwrap().show();
                    stone -= 2;
                    continue 'game;
                }
                true
            } else if input.len() != 2 {
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
            let board = board_human.clone();
            boards.push(board_human);
            stone += 1;
            board.show();
            if let Some(index) = board.win_index() {
                println!("You lose, put {:?}", unpack_index(index));
                if prompt("[M]atta?")? == "M" {
                    if boards.len() > 3 {
                        boards.pop();
                        boards.pop();
                        boards.pop();
                        boards.last().unwrap().show();
                        stone -= 2;
                        continue 'game;
                    }
                }
                break;
            }
            let hand = next(board.clone(), stone);
            if hand.is_none() {
                break;
            }
            let (hand, rate) = hand.unwrap();
            println!(
                "CPU: {:?}, CPU Rate: {}%",
                unpack_index(hand),
                100 - (rate * 100f32) as i32
            );
            boards.push(board.put(hand).unwrap());
            let board_cpu = boards.last().unwrap();
            stone += 1;
            board_cpu.show();
            if board_cpu.win_index().is_some() {
                println!("You win");
                break;
            }
        }
    }
    Ok(())
}
