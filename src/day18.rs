pub(super) fn run() -> Result<(), super::Error> {
	{
		let input = super::read_input_lines::<String>("day18a")?;
		let result = run_inner(input)?;

		println!("18a: {result}");

		assert_eq!(result, 5198);
	}

	{
		let input = super::read_input_lines::<String>("day18b")?;
		let result = run_inner(input)?;

		println!("18b: {result}");

		assert_eq!(result, 1736);
	}

	Ok(())
}

fn run_inner(input: impl Iterator<Item = Result<String, super::Error>>) -> Result<usize, super::Error> {
	let mut tiles: std::collections::BTreeMap<(usize, usize), Tile> = Default::default();
	let mut start = vec![];

	for (y, line) in input.enumerate() {
		let line = line?;
		for (x, c) in line.chars().enumerate() {
			match c {
				'#' => (),
				'.' => { tiles.insert((x, y), Tile::Empty); },
				'@' => { start.push((x, y)); tiles.insert((x, y), Tile::Empty); },
				c @ 'A'..='Z' => { tiles.insert((x, y), Tile::Gate(u8::try_from(c).expect("A..Z fits in u8") - b'A')); },
				c @ 'a'..='z' => { tiles.insert((x, y), Tile::Key(u8::try_from(c).expect("a..z fits in u8") - b'a')); },
				c => return Err(format!("unexpected character in maze: {c:?}").into()),
			};
		}
	}

	let max_x = tiles.keys().map(|(x, _)| *x).max().ok_or("no solution")?;
	let max_y = tiles.keys().map(|(_, y)| *y).max().ok_or("no solution")?;

	let mut neighbors: std::collections::BTreeMap<(usize, usize), std::collections::BTreeMap<(usize, usize), usize>> = Default::default();
	for x in 0..=max_x {
		for y in 0..=max_y {
			let pos = (x, y);

			match tiles.get(&pos) {
				Some(Tile::Empty) if start.contains(&pos) => (),
				Some(Tile::Key(_) | Tile::Gate(_)) => (),
				_ => continue,
			};

			let neighbors = neighbors.entry(pos).or_default();

			let mut visited: std::collections::BTreeMap<(usize, usize), usize> = Default::default();
			visited.insert(pos, 0);

			let mut to_visit: std::collections::VecDeque<((usize, usize), usize)> = Default::default();
			to_visit.push_back(((pos.0 - 1, pos.1), 1));
			to_visit.push_back(((pos.0 + 1, pos.1), 1));
			to_visit.push_back(((pos.0, pos.1 - 1), 1));
			to_visit.push_back(((pos.0, pos.1 + 1), 1));

			while let Some((pos, distance)) = to_visit.pop_front() {
				match visited.entry(pos) {
					std::collections::btree_map::Entry::Vacant(entry) => {
						entry.insert(distance);
					},
					std::collections::btree_map::Entry::Occupied(mut entry) =>
						if *entry.get() < distance {
							continue;
						}
						else {
							entry.insert(distance);
						},
				}

				match tiles.get(&pos) {
					Some(Tile::Empty) => {
						to_visit.push_back(((pos.0 - 1, pos.1), distance + 1));
						to_visit.push_back(((pos.0 + 1, pos.1), distance + 1));
						to_visit.push_back(((pos.0, pos.1 - 1), distance + 1));
						to_visit.push_back(((pos.0, pos.1 + 1), distance + 1));
					},

					Some(Tile::Gate(_) | Tile::Key(_)) => match neighbors.entry(pos) {
						std::collections::btree_map::Entry::Vacant(entry) => {
							entry.insert(distance);
						},
						std::collections::btree_map::Entry::Occupied(mut entry) =>
							if *entry.get() < distance {
								continue;
							}
							else {
								entry.insert(distance);
							},
					},

					None => (),
				}
			}
		}
	}

	let num_keys = tiles.values().filter(|tile| matches!(tile, Tile::Key(_))).count();


	let result = std::sync::atomic::AtomicUsize::new(usize::max_value());

	let best_paths: std::sync::Mutex<std::collections::BTreeMap<(Vec<(usize, usize)>, BitField), usize>> = Default::default();

	rayon::scope(|s| {
		fn solve<'scope>(
			s: &rayon::Scope<'scope>,
			tiles: &'scope std::collections::BTreeMap<(usize, usize), Tile>,
			neighbors: &'scope std::collections::BTreeMap<(usize, usize), std::collections::BTreeMap<(usize, usize), usize>>,
			best_paths: &'scope std::sync::Mutex<std::collections::BTreeMap<(Vec<(usize, usize)>, BitField), usize>>,
			num_keys: usize,
			distance: usize,
			all_pos: &[(usize, usize)],
			keys: BitField,
			result: &'scope std::sync::atomic::AtomicUsize,
		) {
			loop {
				let result_value = result.load(std::sync::atomic::Ordering::Acquire);

				if distance >= result_value {
					return;
				}
				else if keys.len() == num_keys {
					if result.compare_exchange_weak(result_value, distance, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire) == Ok(result_value) {
						return;
					}
				}
				else {
					break;
				}
			}

			match best_paths.lock().unwrap().entry((all_pos.to_owned(), keys)) {
				std::collections::btree_map::Entry::Occupied(mut entry) => {
					if *entry.get() < distance {
						return;
					}

					entry.insert(distance);
				},

				std::collections::btree_map::Entry::Vacant(entry) => {
					entry.insert(distance);
				},
			}

			for (i, &pos) in all_pos.iter().enumerate() {
				let reachable_keys = reachable_keys(tiles, neighbors, pos, keys);

				for (reachable_key_id, (reachable_key_pos, reachable_key_distance)) in reachable_keys {
					if !keys.contains(reachable_key_id) {
						let mut all_pos = all_pos.to_owned();
						all_pos[i] = reachable_key_pos;

						let mut keys = keys;
						keys.insert(reachable_key_id);

						s.spawn(move |s| solve(s, tiles, neighbors, best_paths, num_keys, distance + reachable_key_distance, &all_pos, keys, result));
					}
				}
			}
		}

		s.spawn(|s| solve(s, &tiles, &neighbors, &best_paths, num_keys, 0, &start, Default::default(), &result));
	});

	let result = result.load(std::sync::atomic::Ordering::Acquire);
	Ok(result)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
	Empty,
	Gate(u8),
	Key(u8),
}

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
struct BitField(u32);

impl std::fmt::Debug for BitField {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{")?;

		let mut first = true;
		for i in 0..32 {
			if self.contains(i) {
				if first {
					write!(f, " {}", char::from(i + b'a'))?;
				}
				else {
					write!(f, ", {}", char::from(i + b'a'))?;
					first = true;
				}
			}
		}

		write!(f, " }}")?;

		Ok(())
	}
}

impl BitField {
	fn contains(self, i: u8) -> bool {
		(self.0 & (1_u32 << i)) != 0
	}

	fn insert(&mut self, i: u8) {
		self.0 |= 1_u32 << i;
	}

	fn len(self) -> usize {
		self.0.count_ones() as _
	}
}

fn reachable_keys(
	tiles: &std::collections::BTreeMap<(usize, usize), Tile>,
	neighbors: &std::collections::BTreeMap<(usize, usize), std::collections::BTreeMap<(usize, usize), usize>>,
	pos: (usize, usize),
	already_collected_keys: BitField,
) -> std::collections::BTreeMap<u8, ((usize, usize), usize)> {
	let mut visited: std::collections::BTreeSet<(usize, usize)> = Default::default();
	let mut to_visit: std::collections::VecDeque<((usize, usize), usize)> = Default::default();
	to_visit.push_back((pos, 0));

	let mut result: std::collections::BTreeMap<u8, ((usize, usize), usize)> = Default::default();

	while let Some((pos, distance)) = to_visit.pop_front() {
		if visited.insert(pos) {
			match tiles.get(&pos) {
				Some(Tile::Gate(g)) if !already_collected_keys.contains(*g) => (),
				Some(Tile::Key(k)) if !already_collected_keys.contains(*k) => match result.entry(*k) {
					std::collections::btree_map::Entry::Vacant(entry) => {
						entry.insert((pos, distance));
					},
					std::collections::btree_map::Entry::Occupied(mut entry) =>
						if entry.get().1 < distance {
							continue;
						}
						else {
							entry.insert((pos, distance));
						},
				},
				Some(_) =>
					for (neighbor_pos, neighbor_distance) in neighbors.get(&pos).unwrap() {
						to_visit.push_back((*neighbor_pos, distance + neighbor_distance));
					},
				None => (),
			}
		}
	}

	result
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_run_inner() {
		fn test(input: &str, expected_result: usize) {
			let actual_result = super::run_inner(input.lines().map(|s| Ok(s.to_owned()))).unwrap();
			assert_eq!(expected_result, actual_result);
		}

		test(
			"\
			#########\n\
			#b.A.@.a#\n\
			#########\n\
			",
			8,
		);

		test(
			"\
			########################\n\
			#f.D.E.e.C.b.A.@.a.B.c.#\n\
			######################.#\n\
			#d.....................#\n\
			########################\n\
			",
			86,
		);

		test(
			"\
			########################\n\
			#...............b.C.D.f#\n\
			#.######################\n\
			#.....@.a.B.c.d.A.e.F.g#\n\
			########################\n\
			",
			132,
		);

		test(
			"\
			#################\n\
			#i.G..c...e..H.p#\n\
			########.########\n\
			#j.A..b...f..D.o#\n\
			########@########\n\
			#k.E..a...g..B.n#\n\
			########.########\n\
			#l.F..d...h..C.m#\n\
			#################\n\
			",
			136,
		);

		test(
			"\
			########################\n\
			#@..............ac.GI.b#\n\
			###d#e#f################\n\
			###A#B#C################\n\
			###g#h#i################\n\
			########################\n\
			",
			81,
		);

		test(
			"\
			#######\n\
			#a.#Cd#\n\
			##@#@##\n\
			#######\n\
			##@#@##\n\
			#cB#Ab#\n\
			#######\n\
			",
			8,
		);

		test(
			"\
			###############\n\
			#d.ABC.#.....a#\n\
			######@#@######\n\
			###############\n\
			######@#@######\n\
			#b.....#.....c#\n\
			###############\n\
			",
			24,
		);

		test(
			"\
			#############\n\
			#DcBa.#.GhKl#\n\
			#.###@#@#I###\n\
			#e#d#####j#k#\n\
			###C#@#@###J#\n\
			#fEbA.#.FgHi#\n\
			#############\n\
			",
			32,
		);

		test(
			"\
			#############\n\
			#g#f.D#..h#l#\n\
			#F###e#E###.#\n\
			#dCba@#@BcIJ#\n\
			#############\n\
			#nK.L@#@G...#\n\
			#M###N#H###.#\n\
			#o#m..#i#jk.#\n\
			#############\n\
			",
			72,
		);
	}
}
