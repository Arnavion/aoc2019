use std::convert::TryInto;

pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day23")?.next().ok_or("file is empty")??;
	let ram: crate::intcode::Ram = line.parse()?;

	let computer_channels = spawn_computers(&ram);

	let mut had_activity = false;

	let mut last_activity_time = std::time::Instant::now();
	let mut got_first_nat = false;
	let mut nat = None;
	let mut previous_nat_y: Option<i64> = None;

	'outer: loop {
		for (_, _, output_receiver) in &computer_channels {
			if let Ok(target) = output_receiver.try_recv() {
				last_activity_time = std::time::Instant::now();

				let x = output_receiver.recv().unwrap();
				let y = output_receiver.recv().unwrap();

				if target == 255 {
					nat = Some((x, y));
					if !got_first_nat {
						println!("23a: {y}");

						assert_eq!(y, 23057);

						got_first_nat = true;
					}
				}
				else {
					let target: usize = target.try_into()?;
					let (input_sender, _, _) = computer_channels.get(target).unwrap();
					input_sender.send(x).unwrap();
					input_sender.send(y).unwrap();
				}

				had_activity = true;
			}
		}

		if had_activity {
			had_activity = false;
		}
		else {
			// If we're here, it's because there was no output from any of the computers that needed to be routed this iteration.
			// If all computers are also waiting for input, the network will be considered idle.
			//
			// But we need to give some time for recently transmitted packets to be received by their targets,
			// so wait for 5ms of no activity before considering the network idle.

			let now = std::time::Instant::now();
			if now.duration_since(last_activity_time).as_millis() > 5 {
				let all_waiting_for_input =
					computer_channels.iter()
					.all(|(_, waiting_for_input, _)| waiting_for_input.load(std::sync::atomic::Ordering::Acquire));
				if all_waiting_for_input {
					if let Some((x, y)) = nat.take() {
						if let Some(previous_nat_y) = previous_nat_y {
							if previous_nat_y == y {
								println!("23b: {y}");

								assert_eq!(y, 15156);

								break 'outer;
							}
						}
						previous_nat_y = Some(y);

						let (input_sender, _, _) = &computer_channels[0];
						input_sender.send(x).unwrap();
						input_sender.send(y).unwrap();

						continue 'outer;
					}
				}
			}

			// Don't spin if there was no activity
			std::thread::sleep(std::time::Duration::from_millis(1));
		}
	}

	Ok(())
}

fn spawn_computers(ram: &crate::intcode::Ram) ->
	Vec<(
		std::sync::mpsc::Sender<i64>,
		std::sync::Arc<std::sync::atomic::AtomicBool>,
		std::sync::mpsc::Receiver<i64>,
	)>
{
	(0_i64..50)
		.map(|i| {
			let computer = crate::intcode::Computer::new(ram.clone());

			let (input_sender, input_receiver) = std::sync::mpsc::channel();

			let waiting_for_input = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

			let input = InputChannel {
				receiver: input_receiver,
				waiting_for_input: waiting_for_input.clone(),
			};

			// Give each computer an output sender corresponding to a unique output receiver.
			// This might seem wasteful given that these are *multiple-producer* receivers,
			// but sharing the same channel of i64 between all computers could interleave their output triples
			// and thus produce garbage.
			//
			// Unfortunately there's no simple way to create a Sender<i64> from a Sender<(i64, i64, i64)>.
			// At best, one would have to create an i64 channel, and spawn a thread that reads three values from the Receiver<i64>
			// and pushes them into the Sender<(i64, i64, i64)>
			let (output_sender, output_receiver) = std::sync::mpsc::channel();

			std::thread::spawn(move || {
				fn execute(
					mut computer: crate::intcode::Computer,
					mut input: InputChannel,
					output_sender: &std::sync::mpsc::Sender<i64>,
				) -> Result<(), super::Error> {
					loop {
						match computer.step(&mut input) {
							Ok(Some(target)) => {
								let x = computer.step(std::iter::empty())?.ok_or("EOF")?;
								let y = computer.step(std::iter::empty())?.ok_or("EOF")?;
								output_sender.send(target)?;
								output_sender.send(x)?;
								output_sender.send(y)?;
							},

							Ok(None) => return Ok(()),

							Err(err) => return Err(err),
						}
					}
				}

				let _ = execute(computer, input, &output_sender);
			});

			input_sender.send(i).unwrap(); // Address
			input_sender.send(-1_i64).unwrap(); // No input initially

			(input_sender, waiting_for_input, output_receiver)
		})
		.collect()
}

struct InputChannel {
	receiver: std::sync::mpsc::Receiver<i64>,
	waiting_for_input: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Iterator for InputChannel {
	type Item = i64;

	fn next(&mut self) -> Option<Self::Item> {
		match self.receiver.try_recv() {
			Ok(input) => Some(input),
			Err(std::sync::mpsc::TryRecvError::Empty) => {
				self.waiting_for_input.store(true, std::sync::atomic::Ordering::Release);
				if let Ok(input) = self.receiver.recv() {
					self.waiting_for_input.store(false, std::sync::atomic::Ordering::Release);
					Some(input)
				}
				else {
					None
				}
			},
			Err(std::sync::mpsc::TryRecvError::Disconnected) => None,
		}
	}
}
