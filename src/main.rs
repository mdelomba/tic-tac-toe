use std::io::{self, Read};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    fn print(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Empty => print!("."),
                    Cell::X => print!("X"),
                    Cell::O => print!("O"),
                }
            }
            println!("");
        }
    }

    fn make_move(&mut self, row: usize, col: usize, player: Player) -> bool {
        // This should check if the position provided is valid.
        // If it is then set the board cell to be the player.
        // Otherwise we need to return false
        if self.cells[row][col] == Cell::Empty {
            self.cells[row][col] = match player {
                Player::X => Cell::X,
                Player::O => Cell::O,
            };
            true
        } else {
            false
        }
    }

    fn contains_winner(&self) -> Option<Player> {
        for row in self.cells.iter() {
            if row[0] == row[1] && row[1] == row[2] && row[0] != Cell::Empty {
                return Some(match row[0] {
                    Cell::X => Player::X,
                    Cell::O => Player::O,
                    _ => unreachable!(),
                });
            }
        }

        None
    }

    fn is_full(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        let mut input = String::new();

        board.print();
        println!(
            "It's {:?}'s turn. Select a spot <0-2> <0-2> for <row> <column>",
            current_player
        );

        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if input.len() != 2 || input[0] > 2 || input[1] > 2 {
            println!("Invalid input");
            continue;
        }

        let (row, column) = (input[0], input[1]);

        if !board.make_move(row, column, current_player) {
            println!("Invalid move");
            continue;
        }

        if let Some(winner) = board.contains_winner() {
            board.print();
            println!("Winner is {:?}", winner);
            break;
        }

        if board.is_full() {
            board.print();
            println!("Board is full. It's a tie.");
            break;
        }

        current_player = match current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
