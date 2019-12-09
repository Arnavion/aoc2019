pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day9")?.next().ok_or("file is empty")??;

	let ram: Result<Vec<_>, super::Error> =
		line.split(',')
		.map(|s| Ok(s.parse()?))
		.collect();
	let ram = ram?;

	{
		let mut ram = crate::day2::Ram(ram.clone());
		let output = crate::day2::execute(&mut ram, std::iter::once(1))?;
		let result = *output.last().ok_or("no output")?;

		println!("9a: {}", result);

		assert_eq!(result, 4288078517);
	}

	{
		let mut ram = crate::day2::Ram(ram);
		let output = crate::day2::execute(&mut ram, std::iter::once(2))?;
		let result = *output.last().ok_or("no output")?;

		println!("9b: {}", result);

		assert_eq!(result, 69256);
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_execute_program() {
		fn test(program: &str, expected_output: &[i64]) {
			let ram: Vec<_> = program.split(',').map(|s| s.parse().unwrap()).collect();
			let mut ram = crate::day2::Ram(ram);

			let actual_output = crate::day2::execute(&mut ram, std::iter::empty()).unwrap();
			assert_eq!(expected_output, &*actual_output);
		}

		test("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99", &[
			109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
		]);

		test("1102,34915192,34915192,7,4,7,99,0", &[
			1219070632396864,
		]);

		test("104,1125899906842624,99", &[
			1125899906842624,
		]);
	}
}
