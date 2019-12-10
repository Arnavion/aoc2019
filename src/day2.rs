pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day2")?.next().ok_or("file is empty")??;

	let ram: crate::intcode::Ram = line.parse()?;

	{
		let mut computer = super::intcode::Computer::new(ram.clone());

		*computer.ram.get_mut(1) = 12;
		*computer.ram.get_mut(2) = 2;

		let _ = computer.execute(std::iter::empty())?;

		let result = computer.ram.get(0);

		println!("2a: {}", result);

		assert_eq!(result, 3895705);
	}

	{
		let mut result = None;

		'outer: for noun in 0..=99 {
			for verb in 0..=99 {
				let mut computer = super::intcode::Computer::new(ram.clone());

				*computer.ram.get_mut(1) = noun;
				*computer.ram.get_mut(2) = verb;

				let _ = computer.execute(std::iter::empty())?;

				#[allow(clippy::inconsistent_digit_grouping)]
				{
					if computer.ram.get(0) == 1969_07_20 {
						result = Some((noun, verb));
						break 'outer;
					}
				}
			}
		}

		let (noun, verb) = result.ok_or("no solution")?;
		let result = noun * 100 + verb;

		println!("2b: {}", result);

		assert_eq!(result, 6417);
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

		test("1,10,20,30", &[
			crate::intcode::Instruction::Add(
				crate::intcode::ParameterIn::Position(10),
				crate::intcode::ParameterIn::Position(20),
				crate::intcode::ParameterOut::Position(30),
			),
		]);

		test("1,9,10,3,2,3,11,0,99,30,40,50", &[
			crate::intcode::Instruction::Add(
				crate::intcode::ParameterIn::Position(9),
				crate::intcode::ParameterIn::Position(10),
				crate::intcode::ParameterOut::Position(3),
			),
			crate::intcode::Instruction::Mul(
				crate::intcode::ParameterIn::Position(3),
				crate::intcode::ParameterIn::Position(11),
				crate::intcode::ParameterOut::Position(0),
			),
			crate::intcode::Instruction::Halt,
		]);
	}

	#[test]
	fn test_execute_program() {
		fn test(program: &str, expected: &[i64]) {
			let mut computer = crate::intcode::Computer::new(program.parse().unwrap());

			let _ = computer.execute(std::iter::empty()).unwrap();

			assert_eq!(expected, &*computer.ram.0);
		}

		test("1,9,10,3,2,3,11,0,99,30,40,50", &[
			3500, 9, 10, 70,
			2, 3, 11, 0,
			99,
			30, 40, 50,
		]);

		test("1,0,0,0,99", &[
			2, 0, 0, 0,
			99,
		]);

		test("2,3,0,3,99", &[
			2, 3, 0, 6,
			99,
		]);

		test("2,4,4,5,99,0", &[
			2, 4, 4, 5,
			99,
			9801,
		]);

		test("1,1,1,4,99,5,6,0,99", &[
			30, 1, 1, 4,
			2,
			5, 6, 0, 99,
		]);
	}
}
