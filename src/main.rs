use std::io::{ stdin, Error };
mod table;

fn main() {
    let mut table: table::Table = table::Table::new();
    let mut turn: i32 = 0;

    loop {
        let mut input_buf: String = String::new();

        println!("{}", table.get_table());

        let input_result: Result<usize, Error> = stdin().read_line(&mut input_buf);

        match input_result {
            Ok(_) => {},
            Err(_) => {}
        }

        table.assign(input_buf.trim(), turn);

        if turn == 1 {
            turn = 0
        } else {
            turn = 1
        }
    }

}