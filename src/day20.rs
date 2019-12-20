pub(super) fn run() -> Result<(), super::Error> {
	let input = parse_input(super::read_input_lines::<String>("day20")?)?;

	{
		let result = distance_to_exit(&input)?;

		println!("20a: {}", result);

		assert_eq!(result, 410);
	}

	{
		let result = distance_to_exit_recursive(&input)?;

		println!("20b: {}", result);

		assert_eq!(result, 5084);
	}

	Ok(())
}

struct Input {
	tiles: std::collections::BTreeMap<(usize, usize), Tile>,
	start: (usize, usize),
	teleport_pairs: std::collections::BTreeMap<(usize, usize), (usize, usize)>,
	max_x: usize,
	max_y: usize,
	neighbors: std::collections::BTreeMap<(usize, usize), std::collections::BTreeMap<(usize, usize), usize>>,
}

fn parse_input(input_iter: impl Iterator<Item = Result<String, super::Error>>) -> Result<Input, super::Error> {
	let tiles = {
		let mut input: std::collections::BTreeMap<(usize, usize), char> = Default::default();
		for (y, line) in input_iter.enumerate() {
			let line = line?;
			for (x, c) in line.chars().enumerate() {
				input.insert((x, y), c);
			}
		}

		let tiles: Result<std::collections::BTreeMap<(usize, usize), Tile>, super::Error> =
			input.iter()
			.filter_map(|(&(x, y), &c)| match c {
				' ' | '#' | 'A'..='Z' => None,
				'.' => Some(Ok(((x - 2, y - 2), Tile::Empty))),
				c => Some(Err(format!("unexpected character in maze at {:?}: {:?}", (x - 2, y - 2), c).into())),
			})
			.collect();
		let mut tiles = tiles?;

		for (&(x, y), &c) in &input {
			if let c1 @ 'A'..='Z' = c {
				let (previous_tile, next_tile, id) = match (input.get(&(x + 1, y)), input.get(&(x, y + 1))) {
					(Some(&c2 @ 'A'..='Z'), _) => ((x - 3, y - 2), (x, y - 2), u16::from(c1 as u8 - b'A') * 100 + u16::from(c2 as u8 - b'A')),
					(_, Some(&c2 @ 'A'..='Z')) => ((x - 2, y - 3), (x - 2, y), u16::from(c1 as u8 - b'A') * 100 + u16::from(c2 as u8 - b'A')),
					_ => continue,
				};

				let target_tile =
					if let Some(tile @ Tile::Empty) = tiles.get_mut(&previous_tile) {
						tile
					}
					else if let Some(tile @ Tile::Empty) = tiles.get_mut(&next_tile) {
						tile
					}
					else {
						return Err(format!("two-letter sequence at {:?} does not have empty tile on either side", (x, y)).into());
					};

				let tile = match id {
					0 => Tile::Entrance, // AA
					2525 => Tile::Exit, // ZZ
					id => Tile::Teleport(id),
				};

				*target_tile = tile;
			}
		}

		tiles
	};

	let start =
		tiles.iter()
		.find_map(|(&(x, y), &tile)| if tile == Tile::Entrance { Some((x, y)) } else { None })
		.ok_or("never found start tile")?;

	let mut teleport_pairs: std::collections::BTreeMap<(usize, usize), (usize, usize)> = Default::default();
	for (&pos1, &tile1) in &tiles {
		let id = if let Tile::Teleport(id) = tile1 { id } else { continue; };

		if teleport_pairs.contains_key(&pos1) {
			continue;
		}

		for (&pos2, &tile2) in &tiles {
			match tile2 {
				Tile::Teleport(id2) if id2 == id && pos2 != pos1 => {
					teleport_pairs.insert(pos1, pos2);
					teleport_pairs.insert(pos2, pos1);
				},

				_ => (),
			}
		}
	}

	let max_x = tiles.keys().map(|(x, _)| *x).max().ok_or("no solution")?;
	let max_y = tiles.keys().map(|(_, y)| *y).max().ok_or("no solution")?;

	let mut neighbors: std::collections::BTreeMap<(usize, usize), std::collections::BTreeMap<(usize, usize), usize>> = Default::default();
	for x in 0..=max_x {
		for y in 0..=max_y {
			let pos = (x, y);

			match tiles.get(&pos) {
				Some(Tile::Empty) | None => continue,
				_ => (),
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

					Some(Tile::Entrance) | Some(Tile::Exit) | Some(Tile::Teleport(_)) => match neighbors.entry(pos) {
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

	Ok(Input {
		tiles,
		start,
		teleport_pairs,
		max_x,
		max_y,
		neighbors,
	})
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
	Empty,
	Teleport(u16),
	Entrance,
	Exit,
}

fn distance_to_exit(input: &Input) -> Result<usize, super::Error> {
	#[allow(clippy::unneeded_field_pattern)]
	let Input { tiles, start, teleport_pairs, max_x: _, max_y: _, neighbors } = input;

	let result = std::sync::atomic::AtomicUsize::new(usize::max_value());

	let best_paths: std::sync::Mutex<std::collections::BTreeMap<(usize, usize), usize>> = Default::default();

	rayon::scope(|s| {
		#[allow(clippy::needless_pass_by_value)]
		fn solve<'scope>(
			s: &rayon::Scope<'scope>,
			tiles: &'scope std::collections::BTreeMap<(usize, usize), Tile>,
			teleport_pairs: &'scope std::collections::BTreeMap<(usize, usize), (usize, usize)>,
			neighbors: &'scope std::collections::BTreeMap<(usize, usize), std::collections::BTreeMap<(usize, usize), usize>>,
			best_paths: &'scope std::sync::Mutex<std::collections::BTreeMap<(usize, usize), usize>>,
			distance: usize,
			pos: (usize, usize),
			result: &'scope std::sync::atomic::AtomicUsize,
		) {
			loop {
				let result_value = result.load(std::sync::atomic::Ordering::Acquire);

				if distance >= result_value {
					return;
				}
				else if tiles.get(&pos) == Some(&Tile::Exit) {
					if result.compare_and_swap(result_value, distance, std::sync::atomic::Ordering::AcqRel) == result_value {
						return;
					}
				}
				else {
					break;
				}
			}

			match best_paths.lock().unwrap().entry(pos) {
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

			let reachable =
				reachable(
					tiles,
					neighbors,
					|| true,
					|_| true,
					pos,
				);

			for (reachable_tile_pos, reachable_tile_distance) in reachable {
				let (pos, reachable_tile_distance) = match tiles.get(&reachable_tile_pos) {
					Some(Tile::Teleport(_)) => {
						let other_end_pos = *teleport_pairs.get(&reachable_tile_pos).unwrap();
						(other_end_pos, reachable_tile_distance + 1)
					},

					Some(Tile::Exit) => (reachable_tile_pos, reachable_tile_distance),

					_ => continue,
				};

				s.spawn(move |s| solve(
					s,
					tiles, teleport_pairs, neighbors,
					best_paths,
					distance + reachable_tile_distance, pos,
					result,
				));
			}
		}

		s.spawn(|s| solve(
			s,
			&tiles, &teleport_pairs, &neighbors,
			&best_paths,
			0, *start,
			&result,
		));
	});

	let result = result.load(std::sync::atomic::Ordering::Acquire);
	Ok(result)
}

fn distance_to_exit_recursive(input: &Input) -> Result<usize, super::Error> {
	let Input { tiles, start, teleport_pairs, max_x, max_y, neighbors } = input;

	let result = std::sync::atomic::AtomicUsize::new(usize::max_value());

	let best_paths: std::sync::Mutex<std::collections::BTreeMap<((usize, usize), usize), usize>> = Default::default();

	rayon::scope(|s| {
		#[allow(clippy::needless_pass_by_value)]
		fn solve<'scope>(
			s: &rayon::Scope<'scope>,
			tiles: &'scope std::collections::BTreeMap<(usize, usize), Tile>,
			teleport_pairs: &'scope std::collections::BTreeMap<(usize, usize), (usize, usize)>,
			max_x: usize,
			max_y: usize,
			neighbors: &'scope std::collections::BTreeMap<(usize, usize), std::collections::BTreeMap<(usize, usize), usize>>,
			best_paths: &'scope std::sync::Mutex<std::collections::BTreeMap<((usize, usize), usize), usize>>,
			distance: usize,
			pos: (usize, usize),
			depth: usize,
			result: &'scope std::sync::atomic::AtomicUsize,
		) {
			loop {
				let result_value = result.load(std::sync::atomic::Ordering::Acquire);

				if distance >= result_value {
					return;
				}
				else if tiles.get(&pos) == Some(&Tile::Exit) && depth == 0 {
					if result.compare_and_swap(result_value, distance, std::sync::atomic::Ordering::AcqRel) == result_value {
						return;
					}
				}
				else {
					break;
				}
			}

			match best_paths.lock().unwrap().entry((pos, depth)) {
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

			let reachable =
				reachable(
					tiles,
					neighbors,
					|| depth == 0,
					|pos| depth != 0 || teleport_kind(pos, max_x, max_y) == TeleportKind::Inner,
					pos,
				);

			for (reachable_tile_pos, reachable_tile_distance) in reachable {
				let (pos, depth, reachable_tile_distance) = match tiles.get(&reachable_tile_pos) {
					Some(Tile::Teleport(_)) => {
						let other_end_pos = *teleport_pairs.get(&reachable_tile_pos).unwrap();

						let other_end_depth = match (teleport_kind(reachable_tile_pos, max_x, max_y), depth) {
							(TeleportKind::Inner, depth) => depth + 1,
							(TeleportKind::Outer, 0) => continue,
							(TeleportKind::Outer, depth) => depth - 1,
						};

						(other_end_pos, other_end_depth, reachable_tile_distance + 1)
					},

					Some(Tile::Entrance) if depth > 0 => continue,
					Some(Tile::Exit) if depth > 0 => continue,

					Some(Tile::Exit) => (reachable_tile_pos, depth, reachable_tile_distance),

					_ => continue,
				};

				if depth > 32 {
					// Don't starve shorter solutions by recursing endlessly.
					continue;
				}

				s.spawn(move |s| solve(
					s,
					tiles, teleport_pairs, max_x, max_y, neighbors,
					best_paths,
					distance + reachable_tile_distance, pos, depth,
					result,
				));
			}
		}

		s.spawn(|s| solve(
			s,
			&tiles, &teleport_pairs, *max_x, *max_y, &neighbors,
			&best_paths,
			0, *start, 0,
			&result,
		));
	});

	let result = result.load(std::sync::atomic::Ordering::Acquire);
	Ok(result)
}

fn reachable(
	tiles: &std::collections::BTreeMap<(usize, usize), Tile>,
	neighbors: &std::collections::BTreeMap<(usize, usize), std::collections::BTreeMap<(usize, usize), usize>>,
	mut can_take_exit: impl FnMut() -> bool,
	mut can_take_teleport: impl FnMut((usize, usize)) -> bool,
	pos: (usize, usize),
) -> std::collections::BTreeMap<(usize, usize), usize> {
	let original_pos = pos;

	let mut visited: std::collections::BTreeSet<(usize, usize)> = Default::default();
	let mut to_visit: std::collections::VecDeque<((usize, usize), usize)> = Default::default();
	to_visit.push_back((pos, 0));

	let mut result: std::collections::BTreeMap<(usize, usize), usize> = Default::default();

	while let Some((pos, distance)) = to_visit.pop_front() {
		if visited.insert(pos) {
			match tiles.get(&pos) {
				Some(Tile::Exit) => {
					if !can_take_exit() {
						continue;
					}

					match result.entry(pos) {
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
				},

				Some(Tile::Teleport(_)) if pos != original_pos => {
					if !can_take_teleport(pos) {
						continue;
					}

					match result.entry(pos) {
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum TeleportKind {
	Inner,
	Outer,
}

fn teleport_kind(pos: (usize, usize), max_x: usize, max_y: usize) -> TeleportKind {
	if pos.0 == 0 || pos.1 == 0 || pos.0 == max_x || pos.1 == max_y {
		TeleportKind::Outer
	}
	else {
		TeleportKind::Inner
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_distance_to_exit() {
		fn test(input: &str, expected_distance: usize) {
			let input = super::parse_input(input.lines().map(|s| Ok(s.to_owned()))).unwrap();
			let actual_distance = super::distance_to_exit(&input).unwrap();
			assert_eq!(expected_distance, actual_distance);
		}

		test(
"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
",
			23,
		);

		test(
"                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
",
			58,
		);
	}

	#[test]
	fn test_distance_to_exit_recursive() {
		fn test(input: &str, expected_distance: usize) {
			let input = super::parse_input(input.lines().map(|s| Ok(s.to_owned()))).unwrap();
			let actual_distance = super::distance_to_exit_recursive(&input).unwrap();
			assert_eq!(expected_distance, actual_distance);
		}

		test(
"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
",
			26,
		);

		test(
"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     
",
			396,
		);
	}
}
