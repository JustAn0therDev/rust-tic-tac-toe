mod table;
mod assign;
mod tests;

use std::io::{ stdin, Error };
use assign::Assign;

fn main() {
    let mut table: table::Table = table::Table::new();
    let mut assign: Assign = Assign::Circle;

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

        let assign_result = table.assign(input_buf.trim(), assign);

        match assign_result {
            Ok(_) => {
                assign = if assign == Assign::Circle {
                    Assign::X
                } else {
                    Assign::Circle
                };
            },
            Err(error) => {
                println!("{}", error);
            }
        }
    }

}