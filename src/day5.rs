pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day5")?.next().ok_or("file is empty")??;

	let ram: crate::intcode::Ram = line.parse()?;

	{
		let mut computer = crate::intcode::Computer::new(ram.clone());

		let output = computer.execute(std::iter::once(1))?;
		let result = *output.last().ok_or("no output")?;

		println!("5a: {result}");

		assert_eq!(result, 9654885);
	}

	{
		let mut computer = crate::intcode::Computer::new(ram);

		let output = computer.execute(std::iter::once(5))?;
		let result = *output.last().ok_or("no output")?;

		println!("5b: {result}");

		assert_eq!(result, 7079459);
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_parse_program() {
		fn test(actual: &str, expected: &[crate::intcode::Instruction]) {
			let mut ram = actual.parse().unwrap();

			let mut actual = vec![];
			let mut pc = 0;

			loop {
				let instruction = crate::intcode::Instruction::parse(&mut ram, &mut pc).unwrap();
				actual.push(instruction);
				if let crate::intcode::Instruction::Halt = instruction {
					break;
				}

				if pc == ram.0.len() {
					break;
				}
			}

			assert_eq!(expected, &*actual);
		}

		test("1002,4,3,4", &[
			crate::intcode::Instruction::Mul(
				crate::intcode::ParameterIn::Position(4),
				crate::intcode::ParameterIn::Immediate(3),
				crate::intcode::ParameterOut::Position(4),
			),
		]);
	}

	#[test]
	fn test_execute_program() {
		fn test(program: &str, expected_ram: Option<&[i64]>, input: &[i64], expected_output: &[i64]) {
			let mut computer = crate::intcode::Computer::new(program.parse().unwrap());

			let actual_output = computer.execute(input.iter().copied()).unwrap();

			if let Some(expected_ram) = expected_ram {
				assert_eq!(expected_ram, &*computer.ram.0);
			}

			assert_eq!(expected_output, &*actual_output);
		}

		test(
			"3,0,4,0,99",
			Some(&[
				77, 0,
				4, 0,
				99,
			]),
			&[77],
			&[77],
		);

		test(
			"1002,4,3,4,33",
			Some(&[
				1002, 4, 3, 4,
				99,
			]),
			&[],
			&[],
		);

		test(
			"1101,100,-1,4,0",
			Some(&[
				1101, 100, -1, 4,
				99,
			]),
			&[],
			&[],
		);

		for program in &[
			"3,9,8,9,10,9,4,9,99,-1,8",
			"3,3,1108,-1,8,3,4,3,99",
		] {
			test(program, None, &[8], &[1]);
			test(program, None, &[77], &[0]);
		}

		for program in &[
			"3,9,7,9,10,9,4,9,99,-1,8",
			"3,3,1107,-1,8,3,4,3,99",
		] {
			test(program, None, &[3], &[1]);
			test(program, None, &[77], &[0]);
		}

		for program in &[
			"3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9",
			"3,3,1105,-1,9,1101,0,0,12,4,12,99,1",
		] {
			test(program, None, &[77], &[1]);
			test(program, None, &[0], &[0]);
		}

		{
			let program =
				"3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
			test(
				program,
				None,
				&[3],
				&[999],
			);
			test(
				program,
				None,
				&[8],
				&[1000],
			);
			test(
				program,
				None,
				&[77],
				&[1001],
			);
		}
	}
}
