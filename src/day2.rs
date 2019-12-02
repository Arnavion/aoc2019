pub(super) fn run() -> Result<(), super::Error> {
	{
		let line = super::read_input_lines::<String>("day2")?.next().ok_or("file is empty")??;

		let ram: Result<Vec<_>, super::Error> =
			line.split(',')
			.map(|s| Ok(s.parse::<usize>()?))
			.collect();
		let ram = ram?;
		let mut ram = Ram(ram);

		*ram.get_mut(1)? = 12;
		*ram.get_mut(2)? = 2;

		execute(&mut ram)?;

		let result = ram.get(0)?;

		println!("2a: {}", result);

		assert_eq!(result, 3895705);
	}

	{
		let line = super::read_input_lines::<String>("day2")?.next().ok_or("file is empty")??;

		let mut result = None;

		let ram: Result<Vec<_>, super::Error> =
			line.split(',')
			.map(|s| Ok(s.parse::<usize>()?))
			.collect();
		let ram = ram?;

		'outer: for noun in 0..=99 {
			for verb in 0..=99 {
				let ram = ram.clone();
				let mut ram = Ram(ram);

				*ram.get_mut(1)? = noun;
				*ram.get_mut(2)? = verb;

				execute(&mut ram)?;

				#[allow(clippy::inconsistent_digit_grouping)]
				{
					if ram.get(0)? == 1969_07_20 {
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

fn execute(ram: &mut Ram) -> Result<(), super::Error> {
	let mut pc = 0_usize;

	loop {
		let instruction = Instruction::parse(ram, pc)?;
		instruction.execute(ram, &mut pc)?;
		if let Instruction::Halt = instruction {
			break;
		}
	}

	Ok(())
}

#[derive(Debug)]
struct Ram(Vec<usize>);

impl Ram {
	fn get(&self, index: usize) -> Result<usize, super::Error> {
		Ok(*self.0.get(index).ok_or_else(|| format!("SIGSEGV({})", index))?)
	}

	fn get_mut(&mut self, index: usize) -> Result<&mut usize, super::Error> {
		Ok(self.0.get_mut(index).ok_or_else(|| format!("SIGSEGV({})", index))?)
	}
}

#[derive(Clone, Copy, PartialEq)]
enum Instruction {
	Add(usize, usize, usize),
	Mul(usize, usize, usize),
	Halt,
}

impl std::fmt::Debug for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Instruction::Add(in1, in2, out) => write!(f, "*{} = *{} + *{}", out, in1, in2),
			Instruction::Mul(in1, in2, out) => write!(f, "*{} = *{} * *{}", out, in1, in2),
			Instruction::Halt => write!(f, "!"),
		}
	}
}

impl Instruction {
	fn parse(ram: &Ram, pc: usize) -> Result<Self, super::Error> {
		match ram.get(pc)? {
			1 => {
				let in1 = ram.get(pc + 1)?;
				let in2 = ram.get(pc + 2)?;
				let out = ram.get(pc + 3)?;
				Ok(Instruction::Add(in1, in2, out))
			},

			2 => {
				let in1 = ram.get(pc + 1)?;
				let in2 = ram.get(pc + 2)?;
				let out = ram.get(pc + 3)?;
				Ok(Instruction::Mul(in1, in2, out))
			},

			99 => Ok(Instruction::Halt),

			opcode => Err(format!("SIGILL({})", opcode).into()),
		}
	}

	fn execute(self, ram: &mut Ram, pc: &mut usize) -> Result<(), super::Error> {
		match self {
			Instruction::Add(in1, in2, out) => {
				*ram.get_mut(out)? = ram.get(in1)? + ram.get(in2)?;
				*pc += 4;
				Ok(())
			},

			Instruction::Mul(in1, in2, out) => {
				*ram.get_mut(out)? = ram.get(in1)? * ram.get(in2)?;
				*pc += 4;
				Ok(())
			},

			Instruction::Halt => {
				*pc += 1;
				Ok(())
			},
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_parse_program() {
		fn test(actual: &str, expected: Vec<super::Instruction>) {
			let ram: Vec<_> = actual.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
			let mut ram = super::Ram(ram);

			let mut actual = vec![];
			let mut pc = 0;

			loop {
				let instruction = super::Instruction::parse(&mut ram, pc).unwrap();
				actual.push(instruction);
				if let super::Instruction::Halt = instruction {
					break;
				}

				pc += 4;
				if pc == ram.0.len() {
					break;
				}
			}

			assert_eq!(expected, actual);
		}

		test("1,10,20,30", vec![
			super::Instruction::Add(10, 20, 30),
		]);

		test("1,9,10,3,2,3,11,0,99,30,40,50", vec![
			super::Instruction::Add(9, 10, 3),
			super::Instruction::Mul(3, 11, 0),
			super::Instruction::Halt,
		]);
	}

	#[test]
	fn test_execute_program() {
		fn test(program: &str, expected: &[usize]) {
			let ram: Vec<_> = program.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
			let mut ram = super::Ram(ram);

			super::execute(&mut ram).unwrap();

			assert_eq!(expected, &*ram.0);
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
