mod table;
mod player;

use std::io::{ stdin, Error };
use player::Player;

fn main() {
    let mut table: table::Table = table::Table::new();
    let mut player: Player = Player::One;

    loop {
        let mut input_buf: String = String::new();

        println!("{}", table.get_table());

        if table.game_over() {
            println!("Game over!");
            break;
        }

        let input_result: Result<usize, Error> = stdin().read_line(&mut input_buf);

        match input_result {
            Ok(_) => {},
            Err(_) => {}
        }

        let assign_result = table.assign(input_buf.trim(), player);

        match assign_result {
            Ok(_) => {
                player = if player == Player::One {
                    Player::Two
                } else {
                    Player::One
                };
            },
            Err(error) => {
                println!("{}", error);
            }
        }
    }

}