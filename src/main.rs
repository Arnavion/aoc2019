#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
	clippy::default_trait_access,
	clippy::naive_bytecount,
	clippy::too_many_lines,
	clippy::type_complexity,
	clippy::unreadable_literal,
)]

mod intcode;

macro_rules! main {
	($($mod:ident ,)*) => {
		main! {
			@inner
			{}
			{}
			[$($mod,)*]
		}
	};

	(@inner { $($mods:tt)* } { $($calls:tt)* } []) => {
		$($mods)*

		fn main() -> Result<(), Error> {
			$($calls)*

			Ok(())
		}
	};

	(@inner { $($mods:tt)* } { $($calls:tt)* } [$first:ident , $($rest:ident ,)*]) => {
		main! {
			@inner
			{ $($mods)* mod $first; }
			{ $($calls)* $first::run()?; }
			[$($rest ,)*]
		}
	}
}

main! {
	day17,
	day16,
	day15,
	day14,
	day13,
	day12,
	day11,
	day10,
	day9,
	day8,
	day7,
	day6,
	day5,
	day4,
	day3,
	day2,
	day1,
}

fn read_input_lines<T>(filename: &str) -> Result<impl Iterator<Item = Result<T, Error>>, Error> where T: std::str::FromStr, <T as std::str::FromStr>::Err: Into<Error> {
	let mut path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).to_owned();
	path.push("inputs");
	path.push(filename);
	let inner = std::io::BufReader::new(std::fs::File::open(path)?);
	Ok(Lines::new(inner))
}

struct Error(Box<dyn std::error::Error>, backtrace::Backtrace);

impl std::fmt::Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", self.0)?;

		let mut source = self.0.source();
		while let Some(err) = source {
			writeln!(f, "caused by: {}", err)?;
			source = err.source();
		}

		writeln!(f)?;

		writeln!(f, "{:?}", self.1)?;

		Ok(())
	}
}

impl<E> From<E> for Error where E: Into<Box<dyn std::error::Error>> {
	fn from(err: E) -> Self {
		Error(err.into(), Default::default())
	}
}

struct Lines<T> {
	inner: std::io::BufReader<std::fs::File>,
	buf: String,
	_ty: std::marker::PhantomData<fn() -> T>,
}

impl<T> Lines<T> {
	fn new(inner: std::io::BufReader<std::fs::File>) -> Self {
		Lines {
			inner,
			buf: String::new(),
			_ty: Default::default(),
		}
	}
}

impl<T> Iterator for Lines<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: Into<Error> {
	type Item = Result<T, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		use std::io::BufRead;

		self.buf.clear();

		let read = match self.inner.read_line(&mut self.buf) {
			Ok(read) => read,
			Err(err) => return Some(Err(err.into())),
		};
		if read == 0 {
			return None;
		}

		let buf = self.buf.trim_end();

		let value: T = match buf.parse() {
			Ok(value) => value,
			Err(err) => return Some(Err(err.into())),
		};

		Some(Ok(value))
	}
}
