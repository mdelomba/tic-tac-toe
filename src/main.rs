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

    fn contains_winning(&self) -> Option<Player> {
        for row in self.cells.iter() {
            if row[0] == row[1] && row[1] == row[2] && row[0] != Cell::Empty {
                return Some(match row[0] {
                    Cell::X => Player::X,
                    Cell::O => Player::O,
                    _ => unreachable!(),
                });
            }
        }

        false
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        let mut input = String::new();

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
    }
}
