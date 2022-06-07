pub(super) fn run() -> Result<(), super::Error> {
	const COLS: usize = 25;
	const ROWS: usize = 6;

	let line = super::read_input_lines::<String>("day8")?.next().ok_or("file is empty")??;
	let line = line.as_bytes();

	{
		let result =
			line.chunks(COLS * ROWS)
			.map(|layer| {
				let num_zeros = layer.iter().filter(|&&b| b == b'0').count();
				let num_ones = layer.iter().filter(|&&b| b == b'1').count();
				let num_twos = layer.iter().filter(|&&b| b == b'2').count();
				(num_zeros, num_ones * num_twos)
			})
			.min_by_key(|(num_zeros, _)| *num_zeros)
			.map(|(_, result)| result)
			.ok_or("invalid input")?;

		println!("8a: {result}");

		assert_eq!(result, 1572);
	}

	{
		let mut result = vec![b'2'; COLS * ROWS];
		for layer in line.chunks(COLS * ROWS) {
			for (front, back) in result.iter_mut().zip(layer) {
				if *front == b'2' {
					*front = *back;
				}
			}
		}

		println!("8b:");
		for row in result.chunks(COLS) {
			print!("    ");
			for b in row {
				print!("{}", match b {
					b'0' => "  ",
					b'1' => "\u{2588}\u{2588}",
					_ => return Err("invalid input".into()),
				});
			}
			println!();
		}

		// KYHFE
		assert_eq!(&*result, &b"\
			1001010001100101111011110\
			1010010001100101000010000\
			1100001010111101110011100\
			1010000100100101000010000\
			1010000100100101000010000\
			1001000100100101000011110\
		"[..]);
	}

	Ok(())
}
