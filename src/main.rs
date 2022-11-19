// To make a tic tac toe game, we'll need:
// Players to take turns, one at a time,
// a table that can have its state modified,
// at the end of each turn, check if a player won
// and show the table in its current state. 

// For that, we'll need:
// A table object (or struct, whatever) to handle the changing state.
// Players to take turns
// And a function to show the table's current state.

mod table;

fn main() {
    let mut table: table::Table = table::Table::new();

    table.a1_assigned = true;
    table.b2_assigned = true;
    table.c3_assigned = true;

    println!("{}", table.get_table());
}