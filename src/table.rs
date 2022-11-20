mod assign;
use assign::Assign;

pub struct Table {
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

	fn get_assigned_symbol(opt: &Option<Assign>) -> &'static str {
		match opt {
			Some(assigned) => {
				match assigned {
					Assign::Circle => "O",
					Assign::X => "X",
				}
			},
			None => " "
		} 
	}

	pub fn get_table(&self) -> String {
		let mut table = String::new();

		table.push_str(format!("{} | {} | {} \n", Table::get_assigned_symbol(&self.a1_assigned), Table::get_assigned_symbol(&self.a2_assigned), Table::get_assigned_symbol(&self.a3_assigned)).as_str());
		table.push_str(format!("{} | {} | {} \n", Table::get_assigned_symbol(&self.b1_assigned), Table::get_assigned_symbol(&self.b2_assigned), Table::get_assigned_symbol(&self.b3_assigned)).as_str());
		table.push_str(format!("{} | {} | {} \n", Table::get_assigned_symbol(&self.c1_assigned), Table::get_assigned_symbol(&self.c2_assigned), Table::get_assigned_symbol(&self.c3_assigned)).as_str());

		table
	}

	pub fn assign(&mut self, cell: &str, turn: i32) {
		let assign_value: Assign;

		if turn == 0 {
			assign_value = Assign::Circle;
		} else {
			assign_value = Assign::X;
		}

		match cell {
			"a1" => self.a1_assigned = Some(assign_value),
			"a2" => self.a2_assigned = Some(assign_value),
			"a3" => self.a3_assigned = Some(assign_value),
			"b1" => self.b1_assigned = Some(assign_value),
			"b2" => self.b2_assigned = Some(assign_value),
			"b3" => self.b3_assigned = Some(assign_value),
			"c1" => self.c1_assigned = Some(assign_value),
			"c2" => self.c2_assigned = Some(assign_value),
			"c3" => self.c3_assigned = Some(assign_value),
			_ => { println!("Didn't find anything. Cell: {}", cell) }
		};
	}

}