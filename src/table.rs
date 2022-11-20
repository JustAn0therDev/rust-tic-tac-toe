mod assign;
use assign::Assign;

pub struct Table {
	// TODO: instead of "Assign", these fields should have an Option (or union) 
	// indicating whether the value is a circle or an X.
	pub a1_assigned: Option<Assign>,
	pub a2_assigned: Option<Assign>,
	pub a3_assigned: Option<Assign>,
	pub b1_assigned: Option<Assign>,
	pub b2_assigned: Option<Assign>,
	pub b3_assigned: Option<Assign>,
	pub c1_assigned: Option<Assign>,
	pub c2_assigned: Option<Assign>,
	pub c3_assigned: Option<Assign>,
}

impl Table {
	pub fn new() -> Table {
		Table {
			a1_assigned: None,
			a2_assigned: None,
			a3_assigned: None,
			b1_assigned: None,
			b2_assigned: None,
			b3_assigned: None,
			c1_assigned: None,
			c2_assigned: None,
			c3_assigned: None,
		}
	}

	fn get_assigned_symbol(opt: &Option<Assign>) -> Option<&'static str> {
		match opt {
			Some(assigned) => {
				match assigned {
					assign::Assign::Circle => Some("O"),
					assign::Assign::X => Some("X")
				}
			},
			None => Some(" ")
		} 
	}

	pub fn get_table(&self) -> String {
		let mut table = String::new();

		table.push_str(format!("{} | {} | {} \n", Table::get_assigned_symbol(&self.a1_assigned).unwrap(), Table::get_assigned_symbol(&self.a2_assigned).unwrap(), Table::get_assigned_symbol(&self.a3_assigned).unwrap()).as_str());
		table.push_str(format!("{} | {} | {} \n", Table::get_assigned_symbol(&self.b1_assigned).unwrap(), Table::get_assigned_symbol(&self.b2_assigned).unwrap(), Table::get_assigned_symbol(&self.b3_assigned).unwrap()).as_str());
		table.push_str(format!("{} | {} | {} \n", Table::get_assigned_symbol(&self.c1_assigned).unwrap(), Table::get_assigned_symbol(&self.c2_assigned).unwrap(), Table::get_assigned_symbol(&self.c3_assigned).unwrap()).as_str());

		table
	}

	pub fn assign(&mut self, cell: &str, turn: i32) {
		if cell == "a1" {
			if turn == 1 {
				self.a1_assigned = Some(Assign::Circle);
			} else {
				self.a1_assigned = Some(Assign::X);
			}
		}
	}
}