pub struct Table {
	// TODO: instead of "bool", these fields should have an Option (or union) 
	// indicating whether the value is a circle or an X.
	pub a1_assigned: bool,
	pub a2_assigned: bool,
	pub a3_assigned: bool,
	pub b1_assigned: bool,
	pub b2_assigned: bool,
	pub b3_assigned: bool,
	pub c1_assigned: bool,
	pub c2_assigned: bool,
	pub c3_assigned: bool,
}

impl Table {
	pub fn new() -> Table {
		Table {
			a1_assigned: false,
			a2_assigned: false,
			a3_assigned: false,
			b1_assigned: false,
			b2_assigned: false,
			b3_assigned: false,
			c1_assigned: false,
			c2_assigned: false,
			c3_assigned: false,
		}
	}

	fn get_x(assigned: bool) -> &'static str {
		if assigned {
			return "x"; 
		}

		return " ";
	}

	pub fn get_table(&self) -> String {
		let mut table = String::new();

		table.push_str(format!("{} | {} | {} \n", Table::get_x(self.a1_assigned), Table::get_x(self.a2_assigned), Table::get_x(self.a3_assigned)).as_str());
		table.push_str(format!("{} | {} | {} \n", Table::get_x(self.b1_assigned), Table::get_x(self.b2_assigned), Table::get_x(self.b3_assigned)).as_str());
		table.push_str(format!("{} | {} | {} \n", Table::get_x(self.c1_assigned), Table::get_x(self.c2_assigned), Table::get_x(self.c3_assigned)).as_str());

		table
	}
}