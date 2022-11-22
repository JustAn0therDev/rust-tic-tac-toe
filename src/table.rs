use crate::assign::Assign;

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

		table.push_str("  1 | 2 | 3 \n");
		
		table.push_str(
			format!(
				"A {} | {} | {} \n", 
				Table::get_assigned_symbol(&self.a1_assigned), 
				Table::get_assigned_symbol(&self.a2_assigned), 
				Table::get_assigned_symbol(&self.a3_assigned)
			).as_str());

		table.push_str(
			format!(
				"B {} | {} | {} \n", 
				Table::get_assigned_symbol(&self.b1_assigned), 
				Table::get_assigned_symbol(&self.b2_assigned), 
				Table::get_assigned_symbol(&self.b3_assigned)
			).as_str());
		
		table.push_str(
			format!(
				"C {} | {} | {} \n", 
				Table::get_assigned_symbol(&self.c1_assigned), 
				Table::get_assigned_symbol(&self.c2_assigned), 
				Table::get_assigned_symbol(&self.c3_assigned)
			).as_str());

		table
	}

	pub fn assign(&mut self, cell: &str, assign: Assign) -> Result<(), &str> {
		match cell {
			"a1" => Table::try_assign_value(&mut self.a1_assigned, Some(assign)),
			"a2" => Table::try_assign_value(&mut self.a2_assigned, Some(assign)),
			"a3" => Table::try_assign_value(&mut self.a3_assigned, Some(assign)),
			"b1" => Table::try_assign_value(&mut self.b1_assigned, Some(assign)),
			"b2" => Table::try_assign_value(&mut self.b2_assigned, Some(assign)),
			"b3" => Table::try_assign_value(&mut self.b3_assigned, Some(assign)),
			"c1" => Table::try_assign_value(&mut self.c1_assigned, Some(assign)),
			"c2" => Table::try_assign_value(&mut self.c2_assigned, Some(assign)),
			"c3" => Table::try_assign_value(&mut self.c3_assigned, Some(assign)),
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

	pub fn game_over(&self) -> bool {
		if self.check_game_over_by_first_line() {
			return true;
		}

		if self.check_game_over_by_second_line() {
			return true;
		}

		if self.check_game_over_by_third_line() {
			return true;
		}

		if self.check_game_over_by_first_column() {
			return true;
		}

		if self.check_game_over_by_second_column() {
			return true;
		}

		if self.check_game_over_by_third_column() {
			return true;
		}

		if self.check_game_over_by_diagonal_left_to_right() {
			return true;
		}

		if self.check_game_over_by_diagonal_right_to_left() {
			return true;
		}

		return false;
	}

	fn check_game_over_by_first_line(&self) -> bool {
		let mut game_over: bool = true;

		match self.a1_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.a2_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.a3_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		if game_over {
			return self.a1_assigned == self.a2_assigned && self.a2_assigned == self.a3_assigned;
		}

		false
	}

	fn check_game_over_by_second_line(&self) -> bool {
		let mut game_over: bool = true;

		match self.b1_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.b2_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.b3_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		if game_over {
			return self.b1_assigned == self.b2_assigned && self.b2_assigned == self.b3_assigned;
		}

		false
	}

	fn check_game_over_by_third_line(&self) -> bool {
		let mut game_over: bool = true;

		match self.c1_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.c2_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.c3_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		if game_over {
			return self.c1_assigned == self.c2_assigned && self.c2_assigned == self.c3_assigned;
		}

		false
	}

	fn check_game_over_by_first_column(&self) -> bool {
		let mut game_over: bool = true;

		match self.a1_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.b1_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.c1_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		if game_over {
			return self.a1_assigned == self.b1_assigned && self.b1_assigned == self.c1_assigned;
		}

		false
	}

	fn check_game_over_by_second_column(&self) -> bool {
		let mut game_over: bool = true;

		match self.a2_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.b2_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.c2_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		if game_over {
			return self.a2_assigned == self.b2_assigned && self.b2_assigned == self.c2_assigned;
		}

		false
	}

	fn check_game_over_by_third_column(&self) -> bool {
		let mut game_over: bool = true;

		match self.a3_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.b3_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.c3_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		if game_over {
			return self.a3_assigned == self.b3_assigned && self.b3_assigned == self.c3_assigned;
		}

		false
	}

	fn check_game_over_by_diagonal_left_to_right(&self) -> bool {
		let mut game_over: bool = true;

		match self.a1_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.b2_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.c3_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		if game_over {
			return self.a1_assigned == self.b2_assigned && self.b2_assigned == self.c3_assigned;
		}

		false
	}

	fn check_game_over_by_diagonal_right_to_left(&self) -> bool {
		let mut game_over: bool = true;

		match self.a3_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.b2_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		match self.c1_assigned {
			Some(_) => {},
			None => game_over = false,
		}

		if game_over {
			return self.a3_assigned == self.b2_assigned && self.b2_assigned == self.c1_assigned;
		}

		false
	}

}