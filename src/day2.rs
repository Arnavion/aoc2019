use std::convert::TryInto;

pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day2")?.next().ok_or("file is empty")??;

	let ram: Result<Vec<_>, super::Error> =
		line.split(',')
		.map(|s| Ok(s.parse()?))
		.collect();
	let ram = ram?;

	{
		let mut ram = Ram(ram.clone());

		*ram.get_mut(1)? = 12;
		*ram.get_mut(2)? = 2;

		let _ = execute(&mut ram, std::iter::empty())?;

		let result = ram.get(0)?;

		println!("2a: {}", result);

		assert_eq!(result, 3895705);
	}

	{
		let mut result = None;

		'outer: for noun in 0..=99 {
			for verb in 0..=99 {
				let ram = ram.clone();
				let mut ram = Ram(ram);

				*ram.get_mut(1)? = noun;
				*ram.get_mut(2)? = verb;

				let _ = execute(&mut ram, std::iter::empty())?;

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

pub(crate) fn execute(ram: &mut Ram, input: impl IntoIterator<Item = i64>) -> Result<Vec<i64>, super::Error> {
	let mut input = input.into_iter();
	let mut output = vec![];

	let mut pc = 0_usize;

	loop {
		let instruction = Instruction::parse(ram, &mut pc)?;

		if let Some(out) = instruction.execute(ram, &mut pc, &mut input)? {
			output.push(out);
		}

		if let Instruction::Halt = instruction {
			break;
		}
	}

	Ok(output)
}

#[derive(Debug)]
pub(crate) struct Ram(pub(crate) Vec<i64>);

impl Ram {
	pub(crate) fn get(&self, index: usize) -> Result<i64, super::Error> {
		Ok(*self.0.get(index).ok_or_else(|| format!("SIGSEGV({})", index))?)
	}

	pub(crate) fn get_mut(&mut self, index: usize) -> Result<&mut i64, super::Error> {
		Ok(self.0.get_mut(index).ok_or_else(|| format!("SIGSEGV({})", index))?)
	}
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Instruction {
	Add(Parameter, Parameter, usize),
	Mul(Parameter, Parameter, usize),
	Store(usize),
	Output(Parameter),
	JumpIfTrue(Parameter, Parameter),
	JumpIfFalse(Parameter, Parameter),
	LessThan(Parameter, Parameter, usize),
	Equals(Parameter, Parameter, usize),
	Halt,
}

impl std::fmt::Debug for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Instruction::Add(in1, in2, out) => write!(f, "{:?} <- {:?} + {:?}", out, in1, in2),
			Instruction::Mul(in1, in2, out) => write!(f, "{:?} <- {:?} * {:?}", out, in1, in2),
			Instruction::Store(out) => write!(f, "{:?} <- []", out),
			Instruction::Output(r#in) => write!(f, "[] <- {:?}", r#in),
			Instruction::JumpIfTrue(cond, r#in) => write!(f, "if {:?} != 0 then goto {:?}", cond, r#in),
			Instruction::JumpIfFalse(cond, r#in) => write!(f, "if {:?} == 0 then goto {:?}", cond, r#in),
			Instruction::LessThan(in1, in2, out) => write!(f, "{:?} <- if {:?} < {:?} then 1 else 0", out, in1, in2),
			Instruction::Equals(in1, in2, out) => write!(f, "{:?} <- if {:?} == {:?} then 1 else 0", out, in1, in2),
			Instruction::Halt => write!(f, "!"),
		}
	}
}

impl Instruction {
	pub(crate) fn parse(ram: &Ram, pc: &mut usize) -> Result<Self, super::Error> {
		let opcode = ram.get(*pc)?;
		if opcode < 0 {
			return Err(format!("SIGILL({})", opcode).into());
		}
		*pc += 1;

		let mut parameter_mode = opcode / 100;
		let opcode = opcode % 100;

		macro_rules! parameter {
			() => {{
				let value = ram.get(*pc)?;
				*pc += 1;

				let mode = parameter_mode % 10;
				#[allow(unused_assignments)] { parameter_mode /= 10; }

				Parameter::new(mode.try_into()?, value)?
			}};
		}

		macro_rules! parameter_as_out {
			($ident:ident) => {{
				if let Parameter::Position($ident) = $ident {
					$ident
				}
				else {
					return Err("SIGILL: output parameter does not have position mode".into());
				}
			}};
		}

		match opcode {
			1 => {
				let in1 = parameter!();
				let in2 = parameter!();
				let out = parameter!();
				Ok(Instruction::Add(in1, in2, parameter_as_out!(out)))
			},

			2 => {
				let in1 = parameter!();
				let in2 = parameter!();
				let out = parameter!();
				Ok(Instruction::Mul(in1, in2, parameter_as_out!(out)))
			},

			3 => {
				let out = parameter!();
				Ok(Instruction::Store(parameter_as_out!(out)))
			},

			4 => {
				let r#in = parameter!();
				Ok(Instruction::Output(r#in))
			},

			5 => {
				let cond = parameter!();
				let r#in = parameter!();
				Ok(Instruction::JumpIfTrue(cond, r#in))
			},

			6 => {
				let cond = parameter!();
				let r#in = parameter!();
				Ok(Instruction::JumpIfFalse(cond, r#in))
			},

			7 => {
				let in1 = parameter!();
				let in2 = parameter!();
				let out = parameter!();
				Ok(Instruction::LessThan(in1, in2, parameter_as_out!(out)))
			},

			8 => {
				let in1 = parameter!();
				let in2 = parameter!();
				let out = parameter!();
				Ok(Instruction::Equals(in1, in2, parameter_as_out!(out)))
			},

			99 => Ok(Instruction::Halt),

			opcode => Err(format!("SIGILL({})", opcode).into()),
		}
	}

	fn execute(self, ram: &mut Ram, pc: &mut usize, mut input: impl Iterator<Item = i64>) -> Result<Option<i64>, super::Error> {
		match self {
			Instruction::Add(in1, in2, out) => {
				*ram.get_mut(out)? = in1.get(ram)? + in2.get(ram)?;
				Ok(None)
			},

			Instruction::Mul(in1, in2, out) => {
				*ram.get_mut(out)? = in1.get(ram)? * in2.get(ram)?;
				Ok(None)
			},

			Instruction::Store(out) => {
				*ram.get_mut(out)? = input.next().ok_or("EOF")?;
				Ok(None)
			},

			Instruction::Output(r#in) => {
				let output = r#in.get(ram)?;
				Ok(Some(output))
			},

			Instruction::JumpIfTrue(cond, r#in) => {
				if r#cond.get(ram)? != 0 {
					*pc = r#in.get(ram)?.try_into()?;
				}
				Ok(None)
			},

			Instruction::JumpIfFalse(cond, r#in) => {
				if r#cond.get(ram)? == 0 {
					*pc = r#in.get(ram)?.try_into()?;
				}
				Ok(None)
			},

			Instruction::LessThan(in1, in2, out) => {
				*ram.get_mut(out)? = if in1.get(ram)? < in2.get(ram)? { 1 } else { 0 };
				Ok(None)
			},

			Instruction::Equals(in1, in2, out) => {
				*ram.get_mut(out)? = if in1.get(ram)? == in2.get(ram)? { 1 } else { 0 };
				Ok(None)
			},

			Instruction::Halt =>
				Ok(None),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Parameter {
	Position(usize),
	Immediate(i64),
}

impl Parameter {
	fn new(mode: u8, value: i64) -> Result<Self, super::Error> {
		match mode {
			0 => Ok(Parameter::Position(value.try_into()?)),
			1 => Ok(Parameter::Immediate(value)),
			mode => Err(format!("invalid parameter mode {}", mode).into()),
		}
	}

	fn get(self, ram: &Ram) -> Result<i64, super::Error> {
		match self {
			Parameter::Position(pos) => ram.get(pos),
			Parameter::Immediate(value) => Ok(value),
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_parse_program() {
		fn test(actual: &str, expected: &[super::Instruction]) {
			let ram: Vec<_> = actual.split(',').map(|s| s.parse().unwrap()).collect();
			let mut ram = super::Ram(ram);

			let mut actual = vec![];
			let mut pc = 0;

			loop {
				let instruction = super::Instruction::parse(&mut ram, &mut pc).unwrap();
				actual.push(instruction);
				if let super::Instruction::Halt = instruction {
					break;
				}

				if pc == ram.0.len() {
					break;
				}
			}

			assert_eq!(expected, &*actual);
		}

		test("1,10,20,30", &[
			super::Instruction::Add(super::Parameter::Position(10), super::Parameter::Position(20), 30),
		]);

		test("1,9,10,3,2,3,11,0,99,30,40,50", &[
			super::Instruction::Add(super::Parameter::Position(9), super::Parameter::Position(10), 3),
			super::Instruction::Mul(super::Parameter::Position(3), super::Parameter::Position(11), 0),
			super::Instruction::Halt,
		]);
	}

	#[test]
	fn test_execute_program() {
		fn test(program: &str, expected: &[i64]) {
			let ram: Vec<_> = program.split(',').map(|s| s.parse().unwrap()).collect();
			let mut ram = super::Ram(ram);

			let _ = super::execute(&mut ram, std::iter::empty()).unwrap();

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
