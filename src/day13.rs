use std::convert::TryInto;

// To play breakout, set DAY_13_VIS=1

pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day13")?.next().ok_or("file is empty")??;

	let ram: crate::intcode::Ram = line.parse()?;

	{
		let mut tiles: std::collections::BTreeMap<(i64, i64), Tile> = Default::default();

		let mut computer = crate::intcode::Computer::new(ram.clone());

		while let Some(x) = computer.step(std::iter::empty())? {
			let y = match computer.step(std::iter::empty())? {
				Some(y) => y,
				None => break,
			};

			let tile_id = match computer.step(std::iter::empty())? {
				Some(tile_id) => tile_id,
				None => break,
			};
			let tile = tile_id.try_into()?;

			tiles.insert((x, y), tile);
		}

		let result = tiles.values().filter(|&&tile| tile == Tile::Block).count();

		println!("13a: {result}");

		assert_eq!(result, 318);
	}

	{
		let play_breakout = std::env::var("DAY_13_VIS").is_ok();

		let mut ram = ram;
		*ram.get_mut(0) = 2;

		let mut tiles: std::collections::BTreeMap<(i64, i64), Tile> = Default::default();

		let mut score = 0;

		let mut computer = crate::intcode::Computer::new(ram);

		let mut joystick = 0;

		let stdout = std::io::stdout();
		let mut stdout = stdout.lock();

		if play_breakout {
			render_pre(&mut stdout)?;
		}

		while let Some(x) = computer.step(std::iter::once(joystick))? {
			let y = match computer.step(std::iter::empty())? {
				Some(y) => y,
				None => break,
			};

			let third = match computer.step(std::iter::empty())? {
				Some(tile_id) => tile_id,
				None => break,
			};
			if (x, y) == (-1, 0) {
				score = third;

				if play_breakout {
					render_score(&mut stdout, score)?;
				}
			}
			else {
				let tile = third.try_into()?;
				tiles.insert((x, y), tile);

				if play_breakout {
					render_tile(&mut stdout, x, y, tile)?;
					std::thread::sleep(std::time::Duration::from_millis(5));
				}
			}

			if let Some(((ball_x, _), _)) = tiles.iter().find(|(_, tile)| **tile == Tile::Ball) {
				if let Some(((paddle_x, _), _)) = tiles.iter().find(|(_, tile)| **tile == Tile::HorizontalPaddle) {
					joystick = (ball_x - paddle_x).signum();
				}
			}
		}

		if play_breakout {
			render_post(&mut stdout)?;
		}

		println!("13b: {score}");

		assert_eq!(score, 16309);
	}

	Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
	Empty,
	Wall,
	Block,
	HorizontalPaddle,
	Ball,
}

impl std::convert::TryFrom<i64> for Tile {
	type Error = super::Error;

	fn try_from(i: i64) -> Result<Self, Self::Error> {
		match i {
			0 => Ok(Tile::Empty),
			1 => Ok(Tile::Wall),
			2 => Ok(Tile::Block),
			3 => Ok(Tile::HorizontalPaddle),
			4 => Ok(Tile::Ball),
			tile_id => Err(format!("invalid tile ID {tile_id}").into()),
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

	if tile == Tile::Block {
		let color = (x ^ y) % 8;
		write!(stdout, "\x1B[{};{}m", (color / 4) % 2, color % 4 + 31)?;
	}

	write!(stdout, "\x1B[{};{}H{}\x1B[1;1H", y + 2, x * 3 + 1, match tile {
		Tile::Empty => "   ",
		Tile::Wall => "\u{2591}\u{2591}\u{2591}",
		Tile::Block => "\u{2588}\u{2588}\u{2588}",
		Tile::HorizontalPaddle => "\u{1F030}\u{1F030}\u{1F030}",
		Tile::Ball => " \u{2B24} ",
	})?;

	if tile == Tile::Block {
		write!(stdout, "\x1B[0m")?;
	}

	stdout.flush()?;

	Ok(())
}

fn render_score(stdout: &mut std::io::StdoutLock<'_>, score: i64) -> Result<(), super::Error> {
	use std::io::Write;

	write!(stdout, "\x1B[1;4H{:05}", score)?;
	stdout.flush()?;
	Ok(())
}
