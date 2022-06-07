use std::convert::TryInto;

#[derive(Clone)]
pub(crate) struct Computer {
	pub(crate) ram: Ram,
	pc: usize,
	relative_base: isize,
}

impl Computer {
	pub(crate) fn new(ram: Ram) -> Self {
		Computer {
			ram,
			pc: 0,
			relative_base: 0,
		}
	}

	pub(crate) fn step(&mut self, input: impl IntoIterator<Item = i64>) -> Result<Option<i64>, super::Error> {
		let mut input = input.into_iter();

		loop {
			let instruction = Instruction::parse(&mut self.ram, &mut self.pc)?;

			if let Some(out) = instruction.execute(&mut self.ram, &mut self.pc, &mut self.relative_base, &mut input)? {
				return Ok(Some(out));
			}

			if let Instruction::Halt = instruction {
				return Ok(None);
			}
		}
	}

	pub(crate) fn execute(&mut self, input: impl IntoIterator<Item = i64>) -> Result<Vec<i64>, super::Error> {
		let mut input = input.into_iter();

		let mut result = vec![];

		while let Some(output) = self.step(&mut input)? {
			result.push(output);
		}

		Ok(result)
	}
}

#[derive(Clone, Debug)]
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

impl std::str::FromStr for Ram {
	type Err = super::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let ram: Result<Vec<_>, super::Error> =
			s.split(',')
			.map(|s| Ok(s.parse()?))
			.collect();
		Ok(Ram(ram?))
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
			Instruction::Add(in1, in2, out) => write!(f, "{out:?} <- {in1:?} + {in2:?}"),
			Instruction::Mul(in1, in2, out) => write!(f, "{out:?} <- {in1:?} * {in2:?}"),
			Instruction::Store(out) => write!(f, "{out:?} <- []"),
			Instruction::Output(r#in) => write!(f, "[] <- {in:?}"),
			Instruction::JumpIfTrue(cond, r#in) => write!(f, "if {cond:?} != 0 then goto {in:?}"),
			Instruction::JumpIfFalse(cond, r#in) => write!(f, "if {cond:?} == 0 then goto {in:?}"),
			Instruction::LessThan(in1, in2, out) => write!(f, "{out:?} <- if {in1:?} < {in2:?} then 1 else 0"),
			Instruction::Equals(in1, in2, out) => write!(f, "{out:?} <- if {in1:?} == {in2:?} then 1 else 0"),
			Instruction::SetRelativeBase(r#in) => write!(f, "relative_base <- {in:?}"),
			Instruction::Halt => write!(f, "!"),
		}
	}
}

impl Instruction {
	pub(crate) fn parse(ram: &mut Ram, pc: &mut usize) -> Result<Self, super::Error> {
		let opcode = ram.get(*pc);
		if opcode < 0 {
			return Err(format!("SIGILL({opcode})").into());
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

			opcode => Err(format!("SIGILL({opcode})").into()),
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
			mode => Err(format!("invalid parameter mode {mode}").into()),
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
