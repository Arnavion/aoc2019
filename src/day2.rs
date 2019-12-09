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

		*ram.get_mut(1) = 12;
		*ram.get_mut(2) = 2;

		let _ = execute(&mut ram, std::iter::empty())?;

		let result = ram.get(0);

		println!("2a: {}", result);

		assert_eq!(result, 3895705);
	}

	{
		let mut result = None;

		'outer: for noun in 0..=99 {
			for verb in 0..=99 {
				let ram = ram.clone();
				let mut ram = Ram(ram);

				*ram.get_mut(1) = noun;
				*ram.get_mut(2) = verb;

				let _ = execute(&mut ram, std::iter::empty())?;

				#[allow(clippy::inconsistent_digit_grouping)]
				{
					if ram.get(0) == 1969_07_20 {
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

	let mut result = vec![];

	let mut pc = 0;
	let mut relative_base = 0;

	while let Some(output) = step(ram, &mut pc, &mut relative_base, &mut input)? {
		result.push(output);
	}

	Ok(result)
}

pub(crate) fn step(ram: &mut Ram, pc: &mut usize, relative_base: &mut isize, input: impl IntoIterator<Item = i64>) -> Result<Option<i64>, super::Error> {
	let mut input = input.into_iter();

	loop {
		let instruction = Instruction::parse(ram, pc)?;

		if let Some(out) = instruction.execute(ram, pc, relative_base, &mut input)? {
			return Ok(Some(out));
		}

		if let Instruction::Halt = instruction {
			return Ok(None);
		}
	}
}

#[derive(Debug)]
pub(crate) struct Ram(pub(crate) Vec<i64>);

impl Ram {
	pub(crate) fn get(&mut self, index: usize) -> i64 {
		if index >= self.0.len() {
			self.0.resize(index + 1, 0);
		}

		self.0[index]
	}

	pub(crate) fn get_mut(&mut self, index: usize) -> &mut i64 {
		if index >= self.0.len() {
			self.0.resize(index + 1, 0);
		}

		&mut self.0[index]
	}
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Instruction {
	Add(ParameterIn, ParameterIn, ParameterOut),
	Mul(ParameterIn, ParameterIn, ParameterOut),
	Store(ParameterOut),
	Output(ParameterIn),
	JumpIfTrue(ParameterIn, ParameterIn),
	JumpIfFalse(ParameterIn, ParameterIn),
	LessThan(ParameterIn, ParameterIn, ParameterOut),
	Equals(ParameterIn, ParameterIn, ParameterOut),
	SetRelativeBase(ParameterIn),
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
			Instruction::SetRelativeBase(r#in) => write!(f, "relative_base <- {:?}", r#in),
			Instruction::Halt => write!(f, "!"),
		}
	}
}

impl Instruction {
	pub(crate) fn parse(ram: &mut Ram, pc: &mut usize) -> Result<Self, super::Error> {
		let opcode = ram.get(*pc);
		if opcode < 0 {
			return Err(format!("SIGILL({})", opcode).into());
		}
		*pc += 1;

		let mut parameter_mode = opcode / 100;
		let opcode = opcode % 100;

		macro_rules! parameter {
			() => {{
				let value = ram.get(*pc);
				*pc += 1;

				let mode = parameter_mode % 10;
				#[allow(unused_assignments)] { parameter_mode /= 10; }

				ParameterIn::new(mode.try_into()?, value)?
			}};
		}

		match opcode {
			1 => {
				let in1 = parameter!();
				let in2 = parameter!();
				let out = parameter!().try_into()?;
				Ok(Instruction::Add(in1, in2, out))
			},

			2 => {
				let in1 = parameter!();
				let in2 = parameter!();
				let out = parameter!().try_into()?;
				Ok(Instruction::Mul(in1, in2, out))
			},

			3 => {
				let out = parameter!().try_into()?;
				Ok(Instruction::Store(out))
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
				let out = parameter!().try_into()?;
				Ok(Instruction::LessThan(in1, in2, out))
			},

			8 => {
				let in1 = parameter!();
				let in2 = parameter!();
				let out = parameter!().try_into()?;
				Ok(Instruction::Equals(in1, in2, out))
			},

			9 => {
				let r#in = parameter!();
				Ok(Instruction::SetRelativeBase(r#in))
			},

			99 => Ok(Instruction::Halt),

			opcode => Err(format!("SIGILL({})", opcode).into()),
		}
	}

	fn execute(self, ram: &mut Ram, pc: &mut usize, relative_base: &mut isize, mut input: impl Iterator<Item = i64>) -> Result<Option<i64>, super::Error> {
		match self {
			Instruction::Add(in1, in2, out) => {
				*out.get_mut(ram, *relative_base)? = in1.get(ram, *relative_base)? + in2.get(ram, *relative_base)?;
				Ok(None)
			},

			Instruction::Mul(in1, in2, out) => {
				*out.get_mut(ram, *relative_base)? = in1.get(ram, *relative_base)? * in2.get(ram, *relative_base)?;
				Ok(None)
			},

			Instruction::Store(out) => {
				*out.get_mut(ram, *relative_base)? = input.next().ok_or("EOF")?;
				Ok(None)
			},

			Instruction::Output(r#in) => {
				let output = r#in.get(ram, *relative_base)?;
				Ok(Some(output))
			},

			Instruction::JumpIfTrue(cond, r#in) => {
				if r#cond.get(ram, *relative_base)? != 0 {
					*pc = r#in.get(ram, *relative_base)?.try_into()?;
				}
				Ok(None)
			},

			Instruction::JumpIfFalse(cond, r#in) => {
				if r#cond.get(ram, *relative_base)? == 0 {
					*pc = r#in.get(ram, *relative_base)?.try_into()?;
				}
				Ok(None)
			},

			Instruction::LessThan(in1, in2, out) => {
				*out.get_mut(ram, *relative_base)? = if in1.get(ram, *relative_base)? < in2.get(ram, *relative_base)? { 1 } else { 0 };
				Ok(None)
			},

			Instruction::Equals(in1, in2, out) => {
				*out.get_mut(ram, *relative_base)? = if in1.get(ram, *relative_base)? == in2.get(ram, *relative_base)? { 1 } else { 0 };
				Ok(None)
			},

			Instruction::SetRelativeBase(r#in) => {
				let r#in: isize = r#in.get(ram, *relative_base)?.try_into()?;
				*relative_base += r#in;
				Ok(None)
			},

			Instruction::Halt =>
				Ok(None),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ParameterIn {
	Position(usize),
	Immediate(i64),
	Relative(isize),
}

impl ParameterIn {
	fn new(mode: u8, value: i64) -> Result<Self, super::Error> {
		match mode {
			0 => Ok(ParameterIn::Position(value.try_into()?)),
			1 => Ok(ParameterIn::Immediate(value)),
			2 => Ok(ParameterIn::Relative(value.try_into()?)),
			mode => Err(format!("invalid parameter mode {}", mode).into()),
		}
	}

	fn get(self, ram: &mut Ram, relative_base: isize) -> Result<i64, super::Error> {
		let index = match self {
			ParameterIn::Position(pos) => pos,
			ParameterIn::Immediate(value) => return Ok(value),
			ParameterIn::Relative(offset) => (relative_base + offset).try_into()?,
		};
		Ok(ram.get(index))
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ParameterOut {
	Position(usize),
	Relative(isize),
}

impl ParameterOut {
	fn get_mut(self, ram: &mut Ram, relative_base: isize) -> Result<&mut i64, super::Error> {
		let index = match self {
			ParameterOut::Position(pos) => pos,
			ParameterOut::Relative(offset) => (relative_base + offset).try_into()?,
		};
		Ok(ram.get_mut(index))
	}
}

impl std::convert::TryFrom<ParameterIn> for ParameterOut {
	type Error = super::Error;

	fn try_from(param: ParameterIn) -> Result<Self, Self::Error> {
		match param {
			ParameterIn::Position(pos) => Ok(ParameterOut::Position(pos)),
			ParameterIn::Immediate(_) => Err("SIGILL: output parameter has immediate mode".into()),
			ParameterIn::Relative(offset) => Ok(ParameterOut::Relative(offset)),
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
			super::Instruction::Add(super::ParameterIn::Position(10), super::ParameterIn::Position(20), super::ParameterOut::Position(30)),
		]);

		test("1,9,10,3,2,3,11,0,99,30,40,50", &[
			super::Instruction::Add(super::ParameterIn::Position(9), super::ParameterIn::Position(10), super::ParameterOut::Position(3)),
			super::Instruction::Mul(super::ParameterIn::Position(3), super::ParameterIn::Position(11), super::ParameterOut::Position(0)),
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
