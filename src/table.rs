mod assign;

use assign::Assign;
use crate::player::Player;

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

	fn get_assigned_symbol(opt: &Option<Assign>) -> &str {
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

	pub fn assign(&mut self, cell: &str, player: Player) -> Result<(), &str> {
		let assign_value: Option<Assign>;

		if player == Player::One {
			assign_value = Some(Assign::Circle);
		} else {
			assign_value = Some(Assign::X);
		}

		match cell {
			"a1" => Table::try_assign_value(&mut self.a1_assigned, assign_value),
			"a2" => Table::try_assign_value(&mut self.a2_assigned, assign_value),
			"a3" => Table::try_assign_value(&mut self.a3_assigned, assign_value),
			"b1" => Table::try_assign_value(&mut self.b1_assigned, assign_value),
			"b2" => Table::try_assign_value(&mut self.b2_assigned, assign_value),
			"b3" => Table::try_assign_value(&mut self.b3_assigned, assign_value),
			"c1" => Table::try_assign_value(&mut self.c1_assigned, assign_value),
			"c2" => Table::try_assign_value(&mut self.c2_assigned, assign_value),
			"c3" => Table::try_assign_value(&mut self.c3_assigned, assign_value),
			_ => Err("Didn't find anything! Please try again.")
		}
	}

	fn try_assign_value(cell: &mut Option<Assign>, assign_value: Option<Assign>) -> Result<(), &str> {
		let mut result: Result<(), &str> = Ok(());
		match cell {
			Some(_) => result = Err("This cell already has a value. Please select another!"),
			None => *cell = assign_value
		}

		result
	}

}