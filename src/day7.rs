pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day7")?.next().ok_or("file is empty")??;

	let ram: Result<Vec<_>, super::Error> =
		line.split(',')
		.map(|s| Ok(s.parse::<i64>()?))
		.collect();
	let ram = ram?;

	{
		let mut result = None;
		for amplifier1 in 0..=4 {
			for amplifier2 in 0..=4 {
				for amplifier3 in 0..=4 {
					for amplifier4 in 0..=4 {
						for amplifier5 in 0..=4 {
							let settings: std::collections::BTreeSet<_> = vec![amplifier1, amplifier2, amplifier3, amplifier4, amplifier5].into_iter().collect();
							if settings.len() != 5 { continue; }

							let output = get_output_signal(&ram, &[amplifier1, amplifier2, amplifier3, amplifier4, amplifier5])?;

							result = std::cmp::max(result, Some(output));
						}
					}
				}
			}
		}

		let result = result.ok_or("no solution")?;

		println!("7a: {result}");

		assert_eq!(result, 24625);
	}

	{
		let mut result = None;

		for amplifier1 in 5..=9 {
			for amplifier2 in 5..=9 {
				for amplifier3 in 5..=9 {
					for amplifier4 in 5..=9 {
						for amplifier5 in 5..=9 {
							let settings: std::collections::BTreeSet<_> = vec![amplifier1, amplifier2, amplifier3, amplifier4, amplifier5].into_iter().collect();
							if settings.len() != 5 { continue; }

							let output = get_output_signal2(&ram, &[amplifier1, amplifier2, amplifier3, amplifier4, amplifier5])?;

							result = std::cmp::max(result, Some(output));
						}
					}
				}
			}
		}

		let result = result.ok_or("no solution")?;

		println!("7b: {result}");

		assert_eq!(result, 36497698);
	}

	Ok(())
}

fn get_output_signal(ram: &[i64], settings: &[i64]) -> Result<i64, super::Error> {
	let mut output = 0;

	for &setting in settings {
		output = {
			let mut computer = crate::intcode::Computer::new(crate::intcode::Ram(ram.to_owned()));
			let output = computer.execute(vec![setting, output])?;
			*output.last().ok_or("no output")?
		}
	}

	Ok(output)
}

fn get_output_signal2(ram: &[i64], settings: &[i64]) -> Result<i64, super::Error> {
	let mut output = 0;

	let mut amplifiers: Vec<_> =
		settings.iter()
		.map(|_| crate::intcode::Computer::new(crate::intcode::Ram(ram.to_owned())))
		.collect();
	let mut first_pass = true;

	'outer: loop {
		for (i, (&setting, computer)) in settings.iter().zip(&mut amplifiers).enumerate() {
			output = {
				let input = if first_pass { vec![setting, output].into_iter() } else { vec![output].into_iter() };
				let output = computer.step(input)?;
				match (output, i) {
					(Some(output), _) => output,
					(None, 0) => break 'outer,
					(None, _) => return Err("no output".into()),
				}
			};
		}

		first_pass = false;
	}

	Ok(output)
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_get_output_signal() {
		assert_eq!(
			super::get_output_signal(
				&[
					3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
				],
				&[4, 3, 2, 1, 0],
			).unwrap(),
			43210,
		);

		assert_eq!(
			super::get_output_signal(
				&[
					3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0,
				],
				&[0, 1, 2, 3, 4],
			).unwrap(),
			54321,
		);

		assert_eq!(
			super::get_output_signal(
				&[
					3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
					1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0,
					0, 0,
				],
				&[1, 0, 4, 3, 2],
			).unwrap(),
			65210,
		);
	}

	#[test]
	fn test_get_output_signal2() {
		assert_eq!(
			super::get_output_signal2(
				&[
					3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27,
					4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
				],
				&[9, 8, 7, 6, 5],
			).unwrap(),
			139629729,
		);

		assert_eq!(
			super::get_output_signal2(
				&[
					3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55,
					1005, 55, 26, 1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54,
					0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56,
					1005, 56, 6, 99, 0, 0, 0, 0, 10,
				],
				&[9, 7, 8, 5, 6],
			).unwrap(),
			18216,
		);
	}
}
