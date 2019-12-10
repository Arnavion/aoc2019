pub(super) fn run() -> Result<(), super::Error> {
	const EPSILON: f64 = 0.0001;

	let input = super::read_input_lines::<String>("day10")?;

	// Since the first row of the puzzle input might start with `#`, this input file has an empty line between the header and the puzzle input.
	// So it needs to be skipped.
	let grid: Result<Vec<_>, super::Error> = input.skip(1).collect();
	let grid = grid?;

	let asteroids: Vec<_> =
		grid.iter()
		.enumerate()
		.flat_map(|(row_num, row)| {
			row
			.chars()
			.enumerate()
			.filter_map(move |(col_num, c)| if c == '#' { Some((col_num, row_num)) } else { None })
		})
		.collect();

	let laser = {
		let mut result = None;

		for &candidate in &asteroids {
			// Vec<angle>
			//
			// It's a Vec because f64 can't be elements of BTreeSets (not Ord) nor HashSets (not Eq).
			// Also, we need comparisons to be approximate on EPSILON, so f64's default PartialEq and PartialOrd impls
			// wouldn't work for us anyway.
			let mut visible = vec![];

			for &asteroid in &asteroids {
				if candidate == asteroid {
					continue;
				}

				#[allow(clippy::cast_precision_loss)]
				let delta_y = asteroid.1 as f64 - candidate.1 as f64; // grid's Y increases downwards, so it's backwards from Descartes
				#[allow(clippy::cast_precision_loss)]
				let delta_x = candidate.0 as f64 - asteroid.0 as f64;
				let angle = delta_y.atan2(delta_x);
				if !visible.iter().any(|&a| ((a - angle) as f64).abs() < EPSILON) {
					visible.push(angle);
				}
			}

			result = match result {
				Some((previous_candidate, previous_best)) if previous_best > visible.len() => Some((previous_candidate, previous_best)),
				_ => Some((candidate, visible.len())),
			};
		}

		let result = result.ok_or("no solution")?;

		println!("10a: {}", result.1);

		assert_eq!(result.1, 267);

		result.0
	};

	{
		// Vec<(angle, Vec<(distance, coord)>)>
		//
		// They're Vecs because f64 can't be keys of BTreeMaps (not Ord) nor HashMaps (not Eq).
		// Also, we need comparisons to be approximate on EPSILON, so f64's default PartialEq and PartialOrd impls
		// wouldn't work for us anyway.
		let mut to_zap: Vec<(f64, Vec<(f64, (usize, usize))>)> = vec![];

		for asteroid in asteroids {
			if asteroid == laser {
				continue;
			}

			// Laser starts pointing at pi / 2 and moves clockwise, so transform each asteroid's angle to make a valid sort criteria:
			// bring the angle between 0 and 2*pi, negate it, add pi/2, then bring it back between 0 and 2*pi.
			// Now north is 0, east is pi/2, south is 3*pi/2 and west is 2*pi, which is the sort order we want.
			//
			// There's probably some redundancy in the math that could be simplified, but dammit I'm a programmer not a mathematician.
			// Simplifying the math is the compiler's problem.
			#[allow(clippy::cast_precision_loss)]
			let delta_y = laser.1 as f64 - asteroid.1 as f64; // grid's Y increases downwards, so it's backwards from Descartes
			#[allow(clippy::cast_precision_loss)]
			let delta_x = asteroid.0 as f64 - laser.0 as f64;
			let angle = delta_y.atan2(delta_x);
			let angle = (angle + 2. * std::f64::consts::PI) % (2. * std::f64::consts::PI);
			let angle = -angle;
			let angle = angle + std::f64::consts::PI / 2.;
			let angle = (angle + 2. * std::f64::consts::PI) % (2. * std::f64::consts::PI);

			#[allow(clippy::cast_precision_loss)]
			let distance = (laser.1 as f64 - asteroid.1 as f64).powf(2.) + (asteroid.0 as f64 - laser.0 as f64).powf(2.);

			let equal_angle_asteroids = to_zap.iter_mut().find(|(angle2, _)| (angle - angle2).abs() < EPSILON);
			if let Some((_, v)) = equal_angle_asteroids {
				v.push((distance, asteroid));
			}
			else {
				to_zap.push((angle, vec![(distance, asteroid)]));
			}
		}

		to_zap.sort_by(|(angle1, _), (angle2, _)| angle1.partial_cmp(angle2).unwrap());
		for (_, v) in &mut to_zap {
			// Sort backwards by distance, because we'll be popping elements from the end of the Vec as we visit them
			v.sort_by(|(distance1, _), (distance2, _)| distance1.partial_cmp(distance2).unwrap().reverse());
		}

		let num_to_zap = 200;
		let mut num_zapped = 0;
		let mut j = 0; // Index in the angle map. The closest asteroid at this angle will be zapped next.
		let (x, y) = loop {
			if let Some((_, (x, y))) = to_zap[j].1.pop() {
				num_zapped += 1;
				if num_zapped == num_to_zap {
					break (x, y);
				}
			}

			j = (j + 1) % to_zap.len();
		};

		let result = x * 100 + y;

		println!("10b: {}", result);

		assert_eq!(result, 1309);
	}

	Ok(())
}
