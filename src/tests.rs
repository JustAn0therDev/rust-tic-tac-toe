#[cfg(test)]
mod tests {
	use crate::table::Table;
	use crate::assign::Assign;

	#[test]
	fn get_table_should_return_a_string() {
		let table = Table::new();
		assert_ne!(table.get_table(), "");
	}

	#[test]
	fn assign_should_return_valid_result() {
		let mut table = Table::new();
		let result = match table.assign("a1", Assign::Circle) {
			Ok(()) => true,
			Err(_) => false
		};

		assert!(result);
	}

	#[test]
	fn assign_with_value_in_cell_should_return_error_result() {
		let mut table = Table::new();
		_ = table.assign("a1", Assign::Circle);

		let result = match table.assign("a1", Assign::Circle) {
			Ok(()) => false,
			Err(_) => true
		};

		assert!(result);
	}

	#[test]
	fn first_line_assigned_should_end_game() {
		let mut table = Table::new();
		_ = table.assign("a1", Assign::Circle);

		_ = table.assign("a2", Assign::Circle);

		_ = table.assign("a3", Assign::Circle);

		assert!(table.game_over());
	}

	#[test]
	fn second_line_assigned_should_end_game() {
		let mut table = Table::new();
		_ = table.assign("b1", Assign::Circle);

		_ = table.assign("b2", Assign::Circle);

		_ = table.assign("b3", Assign::Circle);

		assert!(table.game_over());
	}

	#[test]
	fn third_line_assigned_should_end_game() {
		let mut table = Table::new();
		_ = table.assign("c1", Assign::Circle);

		_ = table.assign("c2", Assign::Circle);

		_ = table.assign("c3", Assign::Circle);

		assert!(table.game_over());
	}

	#[test]
	fn first_column_assigned_should_end_game() {
		let mut table = Table::new();
		_ = table.assign("a1", Assign::Circle);

		_ = table.assign("b1", Assign::Circle);

		_ = table.assign("c1", Assign::Circle);

		assert!(table.game_over());
	}

	#[test]
	fn second_column_assigned_should_end_game() {
		let mut table = Table::new();
		_ = table.assign("a2", Assign::Circle);

		_ = table.assign("b2", Assign::Circle);

		_ = table.assign("c2", Assign::Circle);

		assert!(table.game_over());
	}
	
	#[test]
	fn third_column_assigned_should_end_game() {
		let mut table = Table::new();
		_ = table.assign("a3", Assign::Circle);

		_ = table.assign("b3", Assign::Circle);

		_ = table.assign("c3", Assign::Circle);

		assert!(table.game_over());
	}

	#[test]
	fn left_to_right_diagonal_assigned_should_end_game() {
		let mut table = Table::new();
		_ = table.assign("a1", Assign::Circle);

		_ = table.assign("b2", Assign::Circle);

		_ = table.assign("c3", Assign::Circle);

		assert!(table.game_over());
	}

	#[test]
	fn right_to_left_assigned_should_end_game() {
		let mut table = Table::new();
		_ = table.assign("a3", Assign::Circle);

		_ = table.assign("b2", Assign::Circle);

		_ = table.assign("c1", Assign::Circle);

		assert!(table.game_over());
	}
}
