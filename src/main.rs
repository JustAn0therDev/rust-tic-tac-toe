mod table;

fn main() {
    let mut table: table::Table = table::Table::new();

    table.assign("a1", 1);

    println!("{}", table.get_table());
}