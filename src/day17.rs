use std::convert::TryInto;

// To render maze, set DAY_17_VIS=1

pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day17")?.next().ok_or("file is empty")??;
	let mut ram: crate::intcode::Ram = line.parse()?;

	{
		let mut computer = crate::intcode::Computer::new(ram.clone());

		let mut tiles: std::collections::BTreeMap<(i64, i64), Tile> = Default::default();
		let mut robot_pos = None;

		let mut pos = (0, 0);

		while let Some(tile) = computer.step(std::iter::empty())? {
			let tile: u8 = tile.try_into()?;
			match tile {
				b'.' => {
					tiles.insert(pos, Tile::Open);
					pos.0 += 1;
				},
				b'#' => {
					tiles.insert(pos, Tile::Scaffold);
					pos.0 += 1;
				},
				b'^' => {
					tiles.insert(pos, Tile::Scaffold);
					robot_pos = Some(pos);
					pos.0 += 1;
				},
				b'\n' => {
					pos.0 = 0;
					pos.1 += 1;
				},
				_ => unreachable!(),
			}
		}

		let render_maze = std::env::var("DAY_17_VIS").is_ok();
		if render_maze {
			let max_x = tiles.keys().map(|(x, _)| *x).max().ok_or("no solution")?;
			let max_y = tiles.keys().map(|(_, y)| *y).max().ok_or("no solution")?;

			for y in 0..=max_y {
				for x in 0..=max_x {
					print!("{}", match tiles.get(&(x, y)).unwrap_or(&Tile::Open) {
						Tile::Open => ".",
						Tile::Scaffold if robot_pos == Some((x, y)) => "^",
						Tile::Scaffold => "#",
					});
				}
				println!();
			}
		}

		let result: i64 =
			tiles.iter()
			.filter_map(|(&(x, y), &tile)| match (tile, tiles.get(&(x - 1, y)), tiles.get(&(x + 1, y)), tiles.get(&(x, y - 1)), tiles.get(&(x, y + 1))) {
				(Tile::Scaffold, Some(Tile::Scaffold), Some(Tile::Scaffold), Some(Tile::Scaffold), Some(Tile::Scaffold)) => Some(x * y),
				_ => None,
			})
			.sum();

		println!("17a: {result}");

		assert_eq!(result, 11140);
	}

	{
		*ram.get_mut(0) = 2;
		let mut computer = crate::intcode::Computer::new(ram);

		let mut input =
			b"A,A,B,B,C,B,C,B,C,A\nL,10,L,10,R,6\nR,12,L,12,L,12\nL,6,L,10,R,12,R,12\nn\n"
			.iter()
			.copied()
			.map(Into::into);

		let result = loop {
			let output = computer.step(&mut input)?.ok_or("program halted")?;
			// "large, non-ASCII value"
			if output > 127 {
				break output;
			}
		};

		println!("17b: {result}");

		assert_eq!(result, 1113108);
	}

	Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
	Open,
	Scaffold,
}
