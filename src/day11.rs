pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day11")?.next().ok_or("file is empty")??;

	let ram: crate::intcode::Ram = line.parse()?;

	{
		let mut cells: std::collections::BTreeMap<(isize, isize), Color> = Default::default();

		execute(&ram, &mut cells)?;

		let result = cells.len();

		println!("11a: {}", result);

		assert_eq!(result, 2018);
	}

	{
		let mut cells: std::collections::BTreeMap<(isize, isize), Color> = Default::default();
		cells.insert((0, 0), Color::White);

		execute(&ram, &mut cells)?;

		println!("11b:");

		let min_row = *cells.keys().map(|(_, y)| y).min().ok_or("no solution")?;
		let max_row = *cells.keys().map(|(_, y)| y).max().ok_or("no solution")?;
		let min_col = *cells.keys().map(|(x, _)| x).min().ok_or("no solution")?;
		let max_col = *cells.keys().map(|(x, _)| x).max().ok_or("no solution")?;

		for i in (min_row - 1)..=(max_row + 1) {
			for j in (min_col - 1)..=(max_col + 1) {
				match cells.get(&(j, i)) {
					Some(&Color::Black) | None => print!("  "),
					Some(&Color::White) => print!("\u{2588}\u{2588}"),
				}
			}
			println!();
		}

		// APFKRKBR
		assert_eq!(cells, vec![
			((0, 0), Color::Black),
			((0, 3), Color::Black),
			((0, 4), Color::Black),
			((1, 0), Color::Black),
			((1, 1), Color::White),
			((1, 2), Color::White),
			((1, 3), Color::White),
			((1, 4), Color::White),
			((1, 5), Color::White),
			((2, 0), Color::White),
			((2, 1), Color::Black),
			((2, 2), Color::Black),
			((2, 3), Color::White),
			((2, 4), Color::Black),
			((2, 5), Color::Black),
			((3, 0), Color::White),
			((3, 1), Color::Black),
			((3, 2), Color::Black),
			((3, 3), Color::White),
			((3, 4), Color::Black),
			((3, 5), Color::Black),
			((4, 0), Color::Black),
			((4, 1), Color::White),
			((4, 2), Color::White),
			((4, 3), Color::White),
			((4, 4), Color::White),
			((4, 5), Color::White),
			((5, 0), Color::Black),
			((5, 1), Color::Black),
			((5, 2), Color::Black),
			((5, 3), Color::Black),
			((5, 4), Color::Black),
			((5, 5), Color::Black),
			((6, 0), Color::White),
			((6, 1), Color::White),
			((6, 2), Color::White),
			((6, 3), Color::White),
			((6, 4), Color::White),
			((6, 5), Color::White),
			((7, 0), Color::White),
			((7, 1), Color::Black),
			((7, 2), Color::Black),
			((7, 3), Color::White),
			((7, 4), Color::Black),
			((7, 5), Color::Black),
			((8, 0), Color::White),
			((8, 1), Color::Black),
			((8, 2), Color::Black),
			((8, 3), Color::White),
			((8, 4), Color::Black),
			((8, 5), Color::Black),
			((9, 0), Color::Black),
			((9, 1), Color::White),
			((9, 2), Color::White),
			((9, 3), Color::Black),
			((9, 4), Color::Black),
			((9, 5), Color::Black),
			((10, 0), Color::Black),
			((10, 1), Color::Black),
			((10, 2), Color::Black),
			((10, 3), Color::Black),
			((10, 4), Color::Black),
			((10, 5), Color::Black),
			((11, 0), Color::White),
			((11, 1), Color::White),
			((11, 2), Color::White),
			((11, 3), Color::White),
			((11, 4), Color::White),
			((11, 5), Color::White),
			((12, 0), Color::White),
			((12, 1), Color::Black),
			((12, 2), Color::White),
			((12, 3), Color::Black),
			((12, 4), Color::Black),
			((12, 5), Color::Black),
			((13, 0), Color::White),
			((13, 1), Color::Black),
			((13, 2), Color::White),
			((13, 3), Color::Black),
			((13, 4), Color::Black),
			((13, 5), Color::Black),
			((14, 0), Color::White),
			((14, 1), Color::Black),
			((14, 2), Color::Black),
			((14, 3), Color::Black),
			((14, 4), Color::Black),
			((14, 5), Color::Black),
			((15, 0), Color::Black),
			((15, 1), Color::Black),
			((15, 2), Color::Black),
			((15, 3), Color::Black),
			((15, 4), Color::Black),
			((15, 5), Color::Black),
			((16, 0), Color::White),
			((16, 1), Color::White),
			((16, 2), Color::White),
			((16, 3), Color::White),
			((16, 4), Color::White),
			((16, 5), Color::White),
			((17, 0), Color::Black),
			((17, 1), Color::Black),
			((17, 2), Color::White),
			((17, 3), Color::Black),
			((17, 4), Color::Black),
			((17, 5), Color::Black),
			((18, 0), Color::Black),
			((18, 1), Color::White),
			((18, 2), Color::Black),
			((18, 3), Color::White),
			((18, 4), Color::White),
			((18, 5), Color::Black),
			((19, 0), Color::White),
			((19, 1), Color::Black),
			((19, 2), Color::Black),
			((19, 3), Color::Black),
			((19, 4), Color::Black),
			((19, 5), Color::White),
			((20, 0), Color::Black),
			((20, 1), Color::Black),
			((20, 2), Color::Black),
			((20, 3), Color::Black),
			((20, 4), Color::Black),
			((20, 5), Color::Black),
			((21, 0), Color::White),
			((21, 1), Color::White),
			((21, 2), Color::White),
			((21, 3), Color::White),
			((21, 4), Color::White),
			((21, 5), Color::White),
			((22, 0), Color::White),
			((22, 1), Color::Black),
			((22, 2), Color::Black),
			((22, 3), Color::White),
			((22, 4), Color::Black),
			((22, 5), Color::Black),
			((23, 0), Color::White),
			((23, 1), Color::Black),
			((23, 2), Color::Black),
			((23, 3), Color::White),
			((23, 4), Color::White),
			((23, 5), Color::Black),
			((24, 0), Color::Black),
			((24, 1), Color::White),
			((24, 2), Color::White),
			((24, 3), Color::Black),
			((24, 4), Color::Black),
			((24, 5), Color::White),
			((25, 0), Color::Black),
			((25, 1), Color::Black),
			((25, 2), Color::Black),
			((25, 3), Color::Black),
			((25, 4), Color::Black),
			((25, 5), Color::Black),
			((26, 0), Color::White),
			((26, 1), Color::White),
			((26, 2), Color::White),
			((26, 3), Color::White),
			((26, 4), Color::White),
			((26, 5), Color::White),
			((27, 0), Color::Black),
			((27, 1), Color::Black),
			((27, 2), Color::White),
			((27, 3), Color::Black),
			((27, 4), Color::Black),
			((27, 5), Color::Black),
			((28, 0), Color::Black),
			((28, 1), Color::White),
			((28, 2), Color::Black),
			((28, 3), Color::White),
			((28, 4), Color::White),
			((28, 5), Color::Black),
			((29, 0), Color::White),
			((29, 1), Color::Black),
			((29, 2), Color::Black),
			((29, 3), Color::Black),
			((29, 4), Color::Black),
			((29, 5), Color::White),
			((30, 0), Color::Black),
			((30, 1), Color::Black),
			((30, 2), Color::Black),
			((30, 3), Color::Black),
			((30, 4), Color::Black),
			((30, 5), Color::Black),
			((31, 0), Color::White),
			((31, 1), Color::White),
			((31, 2), Color::White),
			((31, 3), Color::White),
			((31, 4), Color::White),
			((31, 5), Color::White),
			((32, 0), Color::White),
			((32, 1), Color::Black),
			((32, 2), Color::White),
			((32, 3), Color::Black),
			((32, 4), Color::Black),
			((32, 5), Color::White),
			((33, 0), Color::White),
			((33, 1), Color::Black),
			((33, 2), Color::White),
			((33, 3), Color::Black),
			((33, 4), Color::Black),
			((33, 5), Color::White),
			((34, 0), Color::Black),
			((34, 1), Color::White),
			((34, 2), Color::Black),
			((34, 3), Color::White),
			((34, 4), Color::White),
			((34, 5), Color::Black),
			((35, 0), Color::Black),
			((35, 1), Color::Black),
			((35, 2), Color::Black),
			((35, 3), Color::Black),
			((35, 4), Color::Black),
			((35, 5), Color::Black),
			((36, 0), Color::White),
			((36, 1), Color::White),
			((36, 2), Color::White),
			((36, 3), Color::White),
			((36, 4), Color::White),
			((36, 5), Color::White),
			((37, 0), Color::White),
			((37, 1), Color::Black),
			((37, 2), Color::Black),
			((37, 3), Color::White),
			((37, 4), Color::Black),
			((37, 5), Color::Black),
			((38, 0), Color::White),
			((38, 1), Color::Black),
			((38, 2), Color::Black),
			((38, 3), Color::White),
			((38, 4), Color::White),
			((38, 5), Color::Black),
			((39, 0), Color::Black),
			((39, 1), Color::White),
			((39, 2), Color::White),
			((39, 3), Color::Black),
			((39, 4), Color::Black),
			((39, 5), Color::White),
			((40, 0), Color::Black),
			((40, 1), Color::Black),
			((40, 2), Color::Black),
			((40, 3), Color::Black),
			((40, 4), Color::Black),
			((40, 5), Color::Black),
			((41, 0), Color::Black),
			((41, 1), Color::Black),
			((41, 2), Color::Black),
			((41, 3), Color::Black),
			((41, 4), Color::Black),
			((42, 1), Color::Black),
			((42, 2), Color::Black),
		].into_iter().collect());
	}

	Ok(())
}

fn execute(ram: &crate::intcode::Ram, cells: &mut std::collections::BTreeMap<(isize, isize), Color>) -> Result<(), super::Error> {
	let mut computer = crate::intcode::Computer::new(ram.clone());

	let mut direction = Direction::Up;
	let mut pos = (0, 0);

	loop {
		let current_color = *cells.entry(pos).or_insert(Color::Black);
		let next_color = computer.step(std::iter::once(match current_color { Color::Black => 0, Color::White => 1 }))?;
		let next_color = match next_color {
			Some(0) => Color::Black,
			Some(1) => Color::White,
			Some(next_color) => return Err(format!("invalid color {}", next_color).into()),
			None => break,
		};
		cells.insert(pos, next_color);

		let turn_order = computer.step(std::iter::empty())?;
		match turn_order {
			Some(0) => direction.turn_left(),
			Some(1) => direction.turn_right(),
			Some(turn_order) => return Err(format!("invalid turn order {}", turn_order).into()),
			None => break,
		}

		direction.advance(&mut pos);
	}

	Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
	Black,
	White,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn turn_left(&mut self) {
		*self = match *self {
			Direction::Up => Direction::Left,
			Direction::Down => Direction::Right,
			Direction::Left => Direction::Down,
			Direction::Right => Direction::Up,
		};
	}

	fn turn_right(&mut self) {
		*self = match *self {
			Direction::Down => Direction::Left,
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Left => Direction::Up,
		};
	}

	fn advance(self, pos: &mut (isize, isize)) {
		match self {
			Direction::Up => { pos.1 -= 1; }
			Direction::Down => { pos.1 += 1; }
			Direction::Left => { pos.0 -= 1; }
			Direction::Right => { pos.0 += 1; }
		}
	}
}
