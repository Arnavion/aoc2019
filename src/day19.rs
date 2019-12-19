pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day19")?.next().ok_or("file is empty")??;
	let ram: crate::intcode::Ram = line.parse()?;

	// Sanity test that beam starts at (0, 0)
	if !test_beam(0, 0, &ram)? {
		return Err("beam does not start at (0, 0)".into());
	}

	{
		let result =
			(0..50)
			.flat_map(|x| (0..50).map(move |y| (x, y)))
			.try_fold(0, |sum, (x, y)| test_beam(x, y, &ram).map(|in_beam| if in_beam { sum + 1 } else { sum }))?;

		println!("19a: {}", result);

		assert_eq!(result, 201);
	}

	{
		// Get estimate about the spread of the beam for a particular y by checking the bounds at y = 100
		//
		// Note: The choice of calculating the slope as x / y instead of the usual y / x is because the beam is taller than it is wide.
		// It makes the calculation of (1) below easier so that invalid dimensions can be filtered out more quickly.
		let (slope_min, slope_max) = {
			let y = 100;

			let mut x_s =
				(0..100)
				.filter_map(|x| match test_beam(x, y, &ram) {
					Ok(true) => Some(Ok(x)),
					Ok(false) => None,
					Err(err) => Some(Err(err)),
				});
			let x_min = x_s.next().ok_or("beam not found at y = 100")??;
			let x_max = x_s.next_back().ok_or("beam not found at y = 100")??;

			#[allow(clippy::cast_precision_loss)]
			let slope_min = x_min as f64 / y as f64;
			#[allow(clippy::cast_precision_loss)]
			let slope_max = x_max as f64 / y as f64;

			// There is inaccuracy from extending from integral to real domain,
			// so it's better to over-estimate the width of the beam than under-estimate it.
			(slope_min * 0.99, slope_max * 1.01)
		};

		let mut result = 0;

		// Starting x will be one where the beam is at least 100 spots high...
		#[allow(clippy::cast_possible_truncation)]
		let y_start_1 = (100. / (slope_max - slope_min)) as i64;
		// ... and at least 100 spots wide
		#[allow(clippy::cast_possible_truncation)]
		let y_start_2 = (100. / (slope_max / slope_min - 1.)) as i64;
		let y_start = std::cmp::max(y_start_1, y_start_2);

		'y: for y in y_start.. {
			// Find all x for this y
			#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
			let x_start = (y as f64 * slope_min) as i64;
			#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
			let x_end = (y as f64 * slope_max) as i64;
			let mut x_s =
				(x_start..x_end)
				.filter_map(|x| match test_beam(x, y, &ram) {
					Ok(true) => Some(Ok(x)),
					Ok(false) => None,
					Err(err) => Some(Err(err)),
				});
			let x_min = x_s.next().ok_or_else(|| format!("beam not found at y = {}", y))??;
			let x_max = x_s.next_back().ok_or_else(|| format!("beam too thin at y = {}", y))??;

			// This y is only a candidate if there are at least 100 x's, and the right-most x has at least 100 spots below it.
			//
			// (1) As mentioned above, the beam is taller than it is wide. So once the beam becomes 100 spots wide enough,
			// it will also have a spot in that line that's 100 spots tall. Thus we can simply verify that the beam is 100 units wide to be sure that at least
			// the right-most spot on that horizontal will also be 100 spots tall.
			//
			// Instead, if we'd first found the spot where the beam becomes tall enough, we would have to search longer to find the spot where it also becomes wide enough.
			//
			// A more general solution would verify which direction the beam grows faster, and dynamically choose whether to stride over x or over y.
			// Then it would work for all inputs, not just my own.
			if x_max - x_min + 1 < 100 {
				continue;
			}
			if !test_beam(x_max, y + 99, &ram)? {
				return Err("expected the beam to be taller than it's wide, and thus already have a spot that's 100 spots tall on this horizontal".into());
			}

			// One of the x's in this y is the solution, except for the 99 x's at the end.

			'x: for x in x_min..=(x_max - 99) {
				// Test the 99th spot below this one
				if !test_beam(x, y + 99, &ram)? {
					continue 'x;
				}

				// Found the spot
				result = x * 10000 + y;
				break 'y;
			}
		}

		println!("19b: {}", result);

		assert_eq!(result, 6610984);
	}

	Ok(())
}

fn test_beam(x: i64, y: i64, ram: &crate::intcode::Ram) -> Result<bool, super::Error> {
	let mut computer = crate::intcode::Computer::new(ram.clone());
	let output = computer.step(std::iter::once(x).chain(std::iter::once(y)))?.ok_or("no output")?;
	match output {
		0 => Ok(false),
		1 => Ok(true),
		output => Err(format!("invalid state {}", output).into()),
	}
}
