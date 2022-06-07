pub(super) fn run() -> Result<(), super::Error> {
	{
		let result =
			super::read_input_lines::<u64>("day1")?
			.try_fold(0, |prev, curr| -> Result<_, super::Error> { Ok(prev + calculate_fuel(curr?)) })?;

		println!("1a: {result}");

		assert_eq!(result, 3318632);
	}

	{
		let result =
			super::read_input_lines::<u64>("day1")?
			.try_fold(0, |prev, curr| -> Result<_, super::Error> { Ok(prev + calculate_fuel_recursive(curr?)) })?;

		println!("1b: {result}");

		assert_eq!(result, 4975084);
	}

	Ok(())
}

fn calculate_fuel(mass: u64) -> u64 {
	(mass / 3).saturating_sub(2)
}

fn calculate_fuel_recursive(mut mass: u64) -> u64 {
	let mut result = 0;

	loop {
		let fuel = calculate_fuel(mass);
		if fuel == 0 {
			break;
		}

		result += fuel;
		mass = fuel;
	}

	result
}

mod tests {
	#[test]
	fn calculate_fuel() {
		assert_eq!(super::calculate_fuel(12), 2);
		assert_eq!(super::calculate_fuel(14), 2);
		assert_eq!(super::calculate_fuel(1969), 654);
		assert_eq!(super::calculate_fuel(100756), 33583);
	}

	#[test]
	fn calculate_fuel_recursive() {
		assert_eq!(super::calculate_fuel_recursive(14), 2);
		assert_eq!(super::calculate_fuel_recursive(1969), 966);
		assert_eq!(super::calculate_fuel_recursive(100756), 50346);
	}
}
