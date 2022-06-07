use std::convert::TryInto;

// To render maze, set DAY_15_VIS=1

pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day15")?.next().ok_or("file is empty")??;

	let ram: crate::intcode::Ram = line.parse()?;

	let tiles = {
		let render_maze = std::env::var("DAY_15_VIS").is_ok();

		// Map of coordinate to tuple of tile type and the distance to that tile from the starting position
		let mut tiles: std::collections::BTreeMap<(i64, i64), (Tile, usize)> = Default::default();
		tiles.insert((0, 0), (Tile::Empty, 0));

		// A list of entries. Each entry contains the coordinate of a tile, the state of the robot at that coordinate,
		// and the distance to that tile from the starting position
		let mut universes: std::collections::VecDeque<((i64, i64), crate::intcode::Computer, usize)> = Default::default();
		universes.push_back(((0, 0), crate::intcode::Computer::new(ram), 0));

		let stdout = std::io::stdout();
		let mut stdout = stdout.lock();

		if render_maze {
			render_pre(&mut stdout)?;
		}

		while let Some((pos, computer, distance)) = universes.pop_front() {
			for &cmd in &[MoveCommand::North, MoveCommand::South, MoveCommand::West, MoveCommand::East] {
				// Try to move in a direction
				let mut pos = pos;
				cmd.advance(&mut pos);

				let distance = distance + 1;

				let mut computer = computer.clone();
				let status = computer.step(std::iter::once(cmd.into()))?.ok_or("program halted")?;
				let status: RobotStatus = status.try_into()?;


				// What did the robot find at the new coordinate?
				match status {
					RobotStatus::HitWall => {
						tiles.insert(pos, (Tile::Wall, 0));

						if render_maze {
							render_tile(&mut stdout, pos.0, pos.1, Tile::Wall)?;
							std::thread::sleep(std::time::Duration::from_millis(10));
						}
					},

					RobotStatus::Moved(o2) => {
						match tiles.entry(pos) {
							std::collections::btree_map::Entry::Occupied(mut entry) => {
								let (_, previous_distance) = entry.get_mut();
								// We already visited this tile before. Since we're doing breadth-first search,
								// the previous visit *must* have had a shorter route to this tile than the current one.
								assert!(distance >= *previous_distance);
							},

							std::collections::btree_map::Entry::Vacant(entry) => {
								// We've reached this tile for the first time.

								let tile = if o2 { Tile::Oxygen } else { Tile::Empty };
								entry.insert((tile, distance));

								if render_maze {
									render_tile(&mut stdout, pos.0, pos.1, tile)?;
									std::thread::sleep(std::time::Duration::from_millis(10));
								}

								// Continue walking the maze from this tile later.
								universes.push_back((pos, computer, distance));
							},
						}
					},
				}
			}
		}

		if render_maze {
			render_post(&mut stdout)?;
		}

		let result =
			tiles.values()
			.find(|(tile, _)| *tile == Tile::Oxygen)
			.map(|(_, distance)| *distance)
			.ok_or("no solution")?;

		println!("15a: {result}");

		assert_eq!(result, 262);

		tiles
	};

	{
		let o2_pos =
			tiles.iter()
			.find(|(_, (tile, _))| *tile == Tile::Oxygen)
			.map(|(pos, _)| *pos)
			.unwrap();

		// Set of tiles that have been visited
		let mut visited_tiles: std::collections::BTreeSet<(i64, i64)> = Default::default();
		let mut result = 0;

		// List of tiles that have not been visited yet, along with the time it took to reach them.
		let mut tiles_to_visit: std::collections::VecDeque<((i64, i64), usize)> = Default::default();
		tiles_to_visit.push_back((o2_pos, 0));

		while let Some((pos, time)) = tiles_to_visit.pop_front() {
			result = std::cmp::max(result, time);

			for &cmd in &[MoveCommand::North, MoveCommand::South, MoveCommand::West, MoveCommand::East] {
				let mut pos = pos;
				cmd.advance(&mut pos);

				match tiles.get(&pos).unwrap() {
					// Only visit empty or oxygen generator tiles
					(Tile::Empty | Tile::Oxygen, _) => {
						if visited_tiles.insert(pos) {
							// Haven't visited this tile before
							tiles_to_visit.push_back((pos, time + 1));
						}
					},

					(Tile::Wall, _) => (),
				}
			}
		}

		println!("15b: {result}");

		assert_eq!(result, 314);
	}

	Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
	Empty,
	Wall,
	Oxygen,
}

#[derive(Clone, Copy, Debug)]
enum MoveCommand {
	North,
	South,
	West,
	East,
}

impl MoveCommand {
	fn advance(self, pos: &mut (i64, i64)) {
		match self {
			MoveCommand::North => { pos.1 -= 1; }
			MoveCommand::South => { pos.1 += 1; }
			MoveCommand::West => { pos.0 -= 1; }
			MoveCommand::East => { pos.0 += 1; }
		}
	}
}

impl std::convert::From<MoveCommand> for i64 {
	fn from(cmd: MoveCommand) -> Self{
		match cmd {
			MoveCommand::North => 1,
			MoveCommand::South => 2,
			MoveCommand::West => 3,
			MoveCommand::East => 4,
		}
	}
}

#[derive(Clone, Copy, Debug)]
enum RobotStatus {
	HitWall,
	Moved(bool),
}

impl std::convert::TryFrom<i64> for RobotStatus {
	type Error = super::Error;

	fn try_from(i: i64) -> Result<Self, Self::Error> {
		match i {
			0 => Ok(RobotStatus::HitWall),
			1 => Ok(RobotStatus::Moved(false)),
			2 => Ok(RobotStatus::Moved(true)),
			i => Err(format!("invalid status {i}").into()),
		}
	}
}

fn render_pre(stdout: &mut std::io::StdoutLock<'_>) -> Result<(), super::Error> {
	use std::io::Write;

	write!(stdout, "\x1B[?1049h\x1B[2J\x1B[3J")?;
	Ok(())
}

fn render_post(stdout: &mut std::io::StdoutLock<'_>) -> Result<(), super::Error> {
	use std::io::Write;

	write!(stdout, "\x1B[?1049l")?;
	Ok(())
}

fn render_tile(stdout: &mut std::io::StdoutLock<'_>, x: i64, y: i64, tile: Tile) -> Result<(), super::Error> {
	use std::io::Write;

	write!(stdout, "\x1B[{};{}H{}\x1B[1;1H", y + 30, x * 2 + 80, match tile {
		Tile::Empty => "  ",
		Tile::Wall => "\u{2591}\u{2591}",
		Tile::Oxygen => "O\u{2082}",
	})?;

	stdout.flush()?;

	Ok(())
}
