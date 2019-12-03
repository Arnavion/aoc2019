pub(super) fn run() -> Result<(), super::Error> {
	let wires: Result<Vec<_>, super::Error> =
		super::read_input_lines::<String>("day3")?
		.map(|line| {
			let line = line?;
			let steps: Result<Vec<Step>, super::Error> = line.split(',').map(str::parse).collect();
			Ok(steps?)
		})
		.collect();
	let wires = wires?;
	let wires: Vec<_> = wires.iter().map(AsRef::as_ref).collect();

	let intersections = find_intersections(wires.iter().copied());

	{
		let result = find_min_manhattan_distance(intersections.iter().copied())?;

		println!("3a: {}", result);

		assert_eq!(result, 860);
	}

	{
		let result =
			find_steps_to_reach(&wires, intersections)
			.map(|(_, num_steps)| num_steps)
			.min()
			.ok_or("no solution")?;

		println!("3b: {}", result);

		assert_eq!(result, 9238);
	}

	Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Step {
	Horizontal(isize),
	Vertical(isize),
}

impl std::str::FromStr for Step {
	type Err = super::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (is_horizontal, direction) = match &s[..1] {
			"D" => (false, -1),
			"L" => (true, -1),
			"R" => (true, 1),
			"U" => (false, 1),
			_ => return Err(format!("malformed step {:?}", s).into()),
		};

		let size: isize = s[1..].parse().map_err(|_| format!("malformed step {:?}", s))?;

		Ok(if is_horizontal { Step::Horizontal(direction * size) } else { Step::Vertical(direction * size) })
	}
}

struct WireWalker<I> {
	steps: I,
	pos: (isize, isize),
	current_step: Option<Step>,
}

impl<I> WireWalker<I> where I: Iterator<Item = Step> {
	fn new(steps: I) -> Self {
		WireWalker {
			steps,
			pos: (0, 0),
			current_step: None,
		}
	}
}

impl<I> Iterator for WireWalker<I> where I: Iterator<Item = Step> {
	type Item = (isize, isize);

	fn next(&mut self) -> Option<Self::Item> {
		let current_step = self.current_step.or_else(|| self.steps.next())?;
		self.current_step = None;

		let (coordinate, distance_sign, next_step) = match current_step {
			Step::Horizontal(distance) if distance.abs() > 0 =>
				(&mut self.pos.0, distance.signum(), Step::Horizontal(distance.signum() * (distance.abs() - 1))),

			Step::Vertical(distance) if distance.abs() > 0 =>
				(&mut self.pos.1, distance.signum(), Step::Vertical(distance.signum() * (distance.abs() - 1))),

			_ => return self.next(),
		};

		*coordinate += distance_sign;

		self.current_step = Some(next_step);

		Some(self.pos)
	}
}

fn find_intersections<'a>(wires: impl IntoIterator<Item = &'a [Step]>) -> std::collections::BTreeSet<(isize, isize)> {
	let wires: Vec<std::collections::BTreeSet<_>> =
		wires.into_iter()
		.map(|steps| WireWalker::new(steps.iter().copied()).collect())
		.collect();

	let mut result = wires[0].clone();
	for wire in &wires[1..] {
		result = result.intersection(wire).copied().collect();
	}

	result
}

fn find_min_manhattan_distance(intersections: impl IntoIterator<Item = (isize, isize)>) -> Result<usize, super::Error> {
	let result =
		intersections.into_iter()
		.map(|(x, y)| x.abs() as usize + y.abs() as usize)
		.min()
		.ok_or("no solution")?;
	Ok(result)
}

fn find_steps_to_reach(
	wires: &[&[Step]],
	intersections: impl IntoIterator<Item = (isize, isize)>,
) -> impl Iterator<Item = ((isize, isize), usize)> {
	let mut steps_to_reach: std::collections::BTreeMap<_, _> =
		intersections.into_iter()
		.map(|pos| (pos, vec![usize::max_value(); wires.len()]))
		.collect();

	for (wire_num, wire) in wires.iter().enumerate() {
		for (step_num, pos) in WireWalker::new(wire.iter().copied()).enumerate() {
			if let Some(steps_to_reach) = steps_to_reach.get_mut(&pos) {
				steps_to_reach[wire_num] = std::cmp::min(steps_to_reach[wire_num], step_num + 1);
			}
		}
	}

	steps_to_reach.into_iter()
		.map(|(pos, steps_to_reach)| (pos, steps_to_reach.into_iter().sum()))
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_parse_steps() {
		fn test(s: &str, expected: &[super::Step]) {
			let actual: Result<Vec<super::Step>, _> = s.split(',').map(str::parse).collect();
			let actual = actual.unwrap();
			assert_eq!(actual, expected);
		}

		test("R8,U5,L5,D3", &[
			super::Step::Horizontal(8),
			super::Step::Vertical(5),
			super::Step::Horizontal(-5),
			super::Step::Vertical(-3),
		]);

		test("U7,R6,D4,L4", &[
			super::Step::Vertical(7),
			super::Step::Horizontal(6),
			super::Step::Vertical(-4),
			super::Step::Horizontal(-4),
		]);
	}

	#[test]
	fn test_min_manhattan_distance() {
		fn test(
			wires: &[&str],
			expected_intersections: Option<&[(isize, isize)]>,
			expected_min_manhattan_distance: usize,
		) {
			let wires: Vec<_> =
				wires.iter().map(|s| {
					let wire: Result<Vec<super::Step>, _> = s.split(',').map(str::parse).collect();
					let wire = wire.unwrap();
					wire
				}).collect();
			let wires: Vec<_> = wires.iter().map(AsRef::as_ref).collect();

			let actual_intersections = super::find_intersections(wires.iter().copied());
			if let Some(expected_intersections) = expected_intersections {
				assert_eq!(actual_intersections, expected_intersections.iter().copied().collect());
			}

			let actual_min_manhattan_distance = super::find_min_manhattan_distance(actual_intersections).unwrap();
			assert_eq!(actual_min_manhattan_distance, expected_min_manhattan_distance);
		}

		test(
			&[
				"R8,U5,L5,D3",
				"U7,R6,D4,L4",
			],
			Some(&[(3, 3), (6, 5)]),
			6,
		);

		test(
			&[
				"R75,D30,R83,U83,L12,D49,R71,U7,L72",
				"U62,R66,U55,R34,D71,R55,D58,R83",
			],
			None,
			159,
		);

		test(
			&[
				"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
				"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
			],
			None,
			135,
		);
	}

	#[test]
	fn test_min_steps_to_reach() {
		fn test(
			wires: &[&str],
			expected_steps_to_reach: Option<&[((isize, isize), usize)]>,
			expected_min_steps_to_reach: usize,
		) {
			let wires: Vec<_> =
				wires.iter().map(|s| {
					let wire: Result<Vec<super::Step>, _> = s.split(',').map(str::parse).collect();
					let wire = wire.unwrap();
					wire
				}).collect();
			let wires: Vec<_> = wires.iter().map(AsRef::as_ref).collect();

			let intersections = super::find_intersections(wires.iter().copied());

			let actual_steps_to_reach: std::collections::BTreeMap<_, _> = super::find_steps_to_reach(&wires, intersections).collect();
			if let Some(expected_steps_to_reach) = expected_steps_to_reach {
				assert_eq!(actual_steps_to_reach, expected_steps_to_reach.iter().copied().collect());
			}

			let actual_min_steps_to_reach = actual_steps_to_reach.values().copied().min().unwrap();
			assert_eq!(actual_min_steps_to_reach, expected_min_steps_to_reach);
		}

		test(
			&[
				"R8,U5,L5,D3",
				"U7,R6,D4,L4",
			],
			Some(&[((3, 3), 40), ((6, 5), 30)]),
			30,
		);

		test(
			&[
				"R75,D30,R83,U83,L12,D49,R71,U7,L72",
				"U62,R66,U55,R34,D71,R55,D58,R83",
			],
			None,
			610,
		);

		test(
			&[
				"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
				"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
			],
			None,
			410,
		);
	}
}
