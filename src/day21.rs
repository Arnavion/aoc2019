// To display output, set DAY_21_VIS=1

pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day21")?.next().ok_or("file is empty")??;
	let ram: crate::intcode::Ram = line.parse()?;

	{
		let result =
			run_inner(
				ram.clone(),
				// The sooner we jump, the more options we'll have available after the jump.
				// But jumping unnecessarily leaves us fewer options by taking away spaces we could've initiated a jump from.
				//
				// Jump now if:
				// - we can land the jump
				// and
				// - there is at least one hole before the jump target
				//
				// Jump = D * (A' + B' + C')
				//      = D * (A * B * C)'
				b"\
					OR A J\n\
					AND B J\n\
					AND C J\n\
					NOT J J\n\
					AND D J\n\
					WALK\n\
				",
			)?;

		println!("21a: {}", result);

		assert_eq!(result, 19361850);
	}

	{
		let result =
			run_inner(
				ram,
				// The sooner we jump, the more options we'll have available after the jump.
				// But jumping unnecessarily leaves us fewer options by taking away spaces we could've initiated a jump from.
				// Also, jumping too soon without looking past the jump leaves us fewer options after the jump.
				//
				// Jump now if:
				// - we can land the jump
				// and
				// - there is at least one hole before the jump target
				// and
				// - * the next space after the jump target is available to be walked on
				//   or
				//   * we could land another jump from the jump target if we wanted to
				//
				// Jump = D * (A' + B' + C') * (E + H)
				//      = D * (A * B * C)' * (E + H)
				b"\
					OR A J\n\
					AND B J\n\
					AND C J\n\
					NOT J J\n\
					AND D J\n\
					OR E T\n\
					OR H T\n\
					AND T J\n\
					RUN\n\
				",
			)?;

		println!("21b: {}", result);

		assert_eq!(result, 1138943788);
	}

	Ok(())
}

fn run_inner(ram: crate::intcode::Ram, input: &[u8]) -> Result<i64, super::Error> {
	let show_output = std::env::var("DAY_21_VIS").is_ok();

	let mut computer = crate::intcode::Computer::new(ram);

	let mut input = input.iter().copied().map(Into::into);

	let mut line = String::new();
	let result = loop {
		let output = computer.step(&mut input)?.ok_or("EOF")?;
		if output > 127 {
			// "a single giant integer outside the normal ASCII range"
			break output;
		}

		#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
		match output as u8 {
			b'\n' => {
				if show_output {
					println!("> {}", line);
				}
				line.clear();
			},

			b => line.push(b as char),
		}
	};

	Ok(result)
}
