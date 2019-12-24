use std::convert::TryInto;

pub(super) fn run() -> Result<(), super::Error> {
	let bugs = parse_input(super::read_input_lines::<String>("day24")?)?;

	{
		let mut bugs = bugs.clone();

		let mut layouts: std::collections::HashSet<std::collections::BTreeSet<(usize, usize)>> = Default::default();
		layouts.insert(bugs.clone());

		let layout = loop {
			let next = evolve(&bugs);

			if layouts.insert(next.clone()) {
				bugs = next;
			}
			else {
				break next;
			}
		};

		let result: usize =
			layout.into_iter()
			.map(|(x, y)| 2_usize.pow((x + y * 5).try_into().unwrap()))
			.sum();

		println!("24a: {}", result);

		assert_eq!(result, 32526865);
	}

	{
		let mut bugs: std::collections::BTreeSet<(isize, usize, usize)> =
			bugs.into_iter()
			.map(|(x, y)| (0, x, y))
			.collect();

		for _ in 0..200 {
			bugs = evolve2(&bugs);
		}

		let result = bugs.len();

		println!("24b: {}", result);

		assert_eq!(result, 2009);
	}

	Ok(())
}

fn parse_input(lines: impl Iterator<Item = Result<String, super::Error>>) -> Result<std::collections::BTreeSet<(usize, usize)>, super::Error> {
	let mut result: std::collections::BTreeSet<(usize, usize)> = Default::default();

	for (y, line) in lines.enumerate() {
		let line = line?;

		assert!(y < 5);

		for (x, c) in line.chars().enumerate() {
			assert!(x < 5);

			if c == '#' {
				result.insert((x, y));
			}
		}
	}

	Ok(result)
}

fn evolve(bugs: &std::collections::BTreeSet<(usize, usize)>) -> std::collections::BTreeSet<(usize, usize)> {
	(0..5)
		.flat_map(|x| (0..5).map(move |y| (x, y)))
		.filter(|&(x, y)| {
			let has_bug = bugs.contains(&(x, y));

			let num_adjacent =
				adjacent((x, y)).iter()
				.filter(|&pos| bugs.contains(pos))
				.count();

			match (has_bug, num_adjacent) {
				(true, 1) |
				(false, 1) |
				(false, 2) => {
					true
				},

				_ => false,
			}
		})
		.collect()
}

fn adjacent((x, y): (usize, usize)) -> &'static [(usize, usize)] {
	static mut RESULT: *mut std::collections::BTreeMap<(usize, usize), &'static [(usize, usize)]> = std::ptr::null_mut();
	static RESULT_INIT: std::sync::Once = std::sync::Once::new();

	RESULT_INIT.call_once(|| {
		let mut result: std::collections::BTreeMap<(usize, usize), &'static [(usize, usize)]> = Default::default();

		for x in 0..5 {
			for y in 0..5 {
				let mut adjacent = vec![];

				// Left
				match (x, y) {
					(0, _) => (),

					(1, _) |
					(2, _) |
					(3, _) |
					(4, _) => {
						adjacent.push((x - 1, y));
					},

					_ => unreachable!(),
				}

				// Right
				match (x, y) {
					(0, _) |
					(1, _) |
					(2, _) |
					(3, _) => {
						adjacent.push((x + 1, y));
					},

					(4, _) => (),

					_ => unreachable!(),
				}

				// Up
				match (x, y) {
					(_, 0) => (),

					(_, 1) |
					(_, 2) |
					(_, 3) |
					(_, 4) => {
						adjacent.push((x, y - 1));
					},

					_ => unreachable!(),
				}

				// Down
				match (x, y) {
					(_, 0) |
					(_, 1) |
					(_, 2) |
					(_, 3) => {
						adjacent.push((x, y + 1));
					},

					(_, 4) => (),

					_ => unreachable!(),
				}

				let adjacent = adjacent.into_boxed_slice();
				let adjacent = Box::leak(adjacent);
				result.insert((x, y), adjacent);
			}
		}

		let result = Box::new(result);
		let result = Box::into_raw(result);
		unsafe { RESULT = result; }
	});

	unsafe { (&*RESULT).get(&(x, y)).unwrap() }
}

fn evolve2(bugs: &std::collections::BTreeSet<(isize, usize, usize)>) -> std::collections::BTreeSet<(isize, usize, usize)> {
	let (min_depth, max_depth) =
		bugs.iter()
		.fold((0, 0), |(min_depth, max_depth), &(depth, _, _)| (std::cmp::min(min_depth, depth), std::cmp::max(max_depth, depth)));

	((min_depth - 1)..=(max_depth + 1))
		.flat_map(|depth| (0..5).map(move |x| (depth, x)))
		.flat_map(|(depth, x)| (0..5).map(move |y| (depth, x, y)))
		.filter(|&(depth, x, y)| {
			if x == 2 && y == 2 {
				return false;
			}

			let has_bug = bugs.contains(&(depth, x, y));

			let num_adjacent =
				adjacent2((x, y)).iter()
				.filter(|&&(depth_delta, x, y)| bugs.contains(&(depth + depth_delta, x, y)))
				.count();

			match (has_bug, num_adjacent) {
				(true, 1) |
				(false, 1) |
				(false, 2) => {
					true
				},

				_ => false,
			}
		})
		.collect()
}

fn adjacent2((x, y): (usize, usize)) -> &'static [(isize, usize, usize)] {
	static mut RESULT: *mut std::collections::BTreeMap<(usize, usize), &'static [(isize, usize, usize)]> = std::ptr::null_mut();
	static RESULT_INIT: std::sync::Once = std::sync::Once::new();

	RESULT_INIT.call_once(|| {
		let mut result: std::collections::BTreeMap<(usize, usize), &'static [(isize, usize, usize)]> = Default::default();

		for x in 0..5 {
			for y in 0..5 {
				let mut adjacent = vec![];

				// Left
				match (x, y) {
					(0, _) => {
						adjacent.push((-1, 1, 2));
					},

					(1, _) |
					(2, _) |
					(3, 0) | (3, 1) | (3, 3) | (3, 4) |
					(4, _) => {
						adjacent.push((0, x - 1, y));
					},

					(3, 2) => {
						adjacent.push((1, 4, 0));
						adjacent.push((1, 4, 1));
						adjacent.push((1, 4, 2));
						adjacent.push((1, 4, 3));
						adjacent.push((1, 4, 4));
					},

					_ => unreachable!(),
				}

				// Right
				match (x, y) {
					(0, _) |
					(1, 0) | (1, 1) | (1, 3) | (1, 4) |
					(2, _) |
					(3, _) => {
						adjacent.push((0, x + 1, y));
					},

					(1, 2) => {
						adjacent.push((1, 0, 0));
						adjacent.push((1, 0, 1));
						adjacent.push((1, 0, 2));
						adjacent.push((1, 0, 3));
						adjacent.push((1, 0, 4));
					},

					(4, _) => {
						adjacent.push((-1, 3, 2));
					},

					_ => unreachable!(),
				}

				// Up
				match (x, y) {
					(_, 0) => {
						adjacent.push((-1, 2, 1));
					},

					(_, 1) |
					(_, 2) |
					(0, 3) | (1, 3) | (3, 3) | (4, 3) |
					(_, 4) => {
						adjacent.push((0, x, y - 1));
					},

					(2, 3) => {
						adjacent.push((1, 0, 4));
						adjacent.push((1, 1, 4));
						adjacent.push((1, 2, 4));
						adjacent.push((1, 3, 4));
						adjacent.push((1, 4, 4));
					},

					_ => unreachable!(),
				}

				// Down
				match (x, y) {
					(_, 0) |
					(0, 1) | (1, 1) | (3, 1) | (4, 1) |
					(_, 2) |
					(_, 3) => {
						adjacent.push((0, x, y + 1));
					},

					(2, 1) => {
						adjacent.push((1, 0, 0));
						adjacent.push((1, 1, 0));
						adjacent.push((1, 2, 0));
						adjacent.push((1, 3, 0));
						adjacent.push((1, 4, 0));
					},

					(_, 4) => {
						adjacent.push((-1, 2, 3));
					},

					_ => unreachable!(),
				}

				let adjacent = adjacent.into_boxed_slice();
				let adjacent = Box::leak(adjacent);
				result.insert((x, y), adjacent);
			}
		}

		let result = Box::new(result);
		let result = Box::into_raw(result);
		unsafe { RESULT = result; }
	});

	unsafe { (&*RESULT).get(&(x, y)).unwrap() }
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_evolve() {
		let input = "\
			....#\n\
			#..#.\n\
			#..##\n\
			..#..\n\
			#....\n\
		";
		let input = super::parse_input(input.lines().map(|s| Ok(s.to_owned()))).unwrap();

		let expected_iteration_1 = "\
			#..#.\n\
			####.\n\
			###.#\n\
			##.##\n\
			.##..\n\
		";
		let expected_iteration_1 = super::parse_input(expected_iteration_1.lines().map(|s| Ok(s.to_owned()))).unwrap();
		let actual_iteration_1 = super::evolve(&input);
		assert_eq!(expected_iteration_1, actual_iteration_1);

		let expected_iteration_2 = "\
			#####\n\
			....#\n\
			....#\n\
			...#.\n\
			#.###\n\
		";
		let expected_iteration_2 = super::parse_input(expected_iteration_2.lines().map(|s| Ok(s.to_owned()))).unwrap();
		let actual_iteration_2 = super::evolve(&actual_iteration_1);
		assert_eq!(expected_iteration_2, actual_iteration_2);

		let expected_iteration_3 = "\
			#....\n\
			####.\n\
			...##\n\
			#.##.\n\
			.##.#\n\
		";
		let expected_iteration_3 = super::parse_input(expected_iteration_3.lines().map(|s| Ok(s.to_owned()))).unwrap();
		let actual_iteration_3 = super::evolve(&actual_iteration_2);
		assert_eq!(expected_iteration_3, actual_iteration_3);

		let expected_iteration_4 = "\
			####.\n\
			....#\n\
			##..#\n\
			.....\n\
			##...\n\
		";
		let expected_iteration_4 = super::parse_input(expected_iteration_4.lines().map(|s| Ok(s.to_owned()))).unwrap();
		let actual_iteration_4 = super::evolve(&actual_iteration_3);
		assert_eq!(expected_iteration_4, actual_iteration_4);
	}

	#[test]
	fn test_evolve2() {
		let input = "\
			....#\n\
			#..#.\n\
			#..##\n\
			..#..\n\
			#....\n\
		";
		let input = super::parse_input(input.lines().map(|s| Ok(s.to_owned()))).unwrap();
		let input: std::collections::BTreeSet<(isize, usize, usize)> =
			input.into_iter()
			.map(|(x, y)| (0, x, y))
			.collect();

		let expected_iteration_10 = &[
			(-5, "\
				..#..\n\
				.#.#.\n\
				..?.#\n\
				.#.#.\n\
				..#..\n\
			"),
			(-4, "\
				...#.\n\
				...##\n\
				..?..\n\
				...##\n\
				...#.\n\
			"),
			(-3, "\
				#.#..\n\
				.#...\n\
				..?..\n\
				.#...\n\
				#.#..\n\
			"),
			(-2, "\
				.#.##\n\
				....#\n\
				..?.#\n\
				...##\n\
				.###.\n\
			"),
			(-1, "\
				#..##\n\
				...##\n\
				..?..\n\
				...#.\n\
				.####\n\
			"),
			(0, "\
				.#...\n\
				.#.##\n\
				.#?..\n\
				.....\n\
				.....\n\
			"),
			(1, "\
				.##..\n\
				#..##\n\
				..?.#\n\
				##.##\n\
				#####\n\
			"),
			(2, "\
				###..\n\
				##.#.\n\
				#.?..\n\
				.#.##\n\
				#.#..\n\
			"),
			(3, "\
				..###\n\
				.....\n\
				#.?..\n\
				#....\n\
				#...#\n\
			"),
			(4, "\
				.###.\n\
				#..#.\n\
				#.?..\n\
				##.#.\n\
				.....\n\
			"),
			(5, "\
				####.\n\
				#..#.\n\
				#.?#.\n\
				####.\n\
				.....\n\
			"),
		];
		let expected_iteration_10: std::collections::BTreeSet<(isize, usize, usize)> =
			expected_iteration_10.iter()
			.flat_map(|&(depth, input)|
				super::parse_input(input.lines().map(|s| Ok(s.to_owned()))).unwrap()
				.into_iter()
				.map(move |(x, y)| (depth, x, y))
			)
			.collect();
		let mut actual_iteration_10 = input;
		for _ in 0..10 {
			actual_iteration_10 = super::evolve2(&actual_iteration_10);
		}
		assert_eq!(expected_iteration_10, actual_iteration_10);
	}
}
