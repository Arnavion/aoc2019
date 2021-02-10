pub(super) fn run() -> Result<(), super::Error> {
	let mut states_x = vec![];
	let mut states_y = vec![];
	let mut states_z = vec![];

	for line in super::read_input_lines::<String>("day12")? {
		let line = line?;
		let line = &line[1..(line.len() - 1)];
		let mut parts = line.split(", ");

		let x = parts.next().ok_or("malformed input")?;
		let x: i64 = x[2..].parse()?;

		let y = parts.next().ok_or("malformed input")?;
		let y: i64 = y[2..].parse()?;

		let z = parts.next().ok_or("malformed input")?;
		let z: i64 = z[2..].parse()?;

		states_x.push(State { r: x, v: 0 });
		states_y.push(State { r: y, v: 0 });
		states_z.push(State { r: z, v: 0 });
	}

	{
		for _ in 0..1000 {
			step(&mut states_x);
			step(&mut states_y);
			step(&mut states_z);
		}

		let result = total_energy(&states_x, &states_y, &states_z);

		println!("12a: {}", result);

		assert_eq!(result, 9139);
	}

	{
		// There's no pressing need to use the original unmodified states_{x,y,z} here instead of the ones stepped forward by
		// 1000 steps in part 1, because they'll have the cycles anyway.

		let x_cycle_len = find_cycle_len(&mut states_x.clone(), &mut states_x);
		let y_cycle_len = find_cycle_len(&mut states_y.clone(), &mut states_y);
		let z_cycle_len = find_cycle_len(&mut states_z.clone(), &mut states_z);

		// Use the right tool for the job
		println!("12b: https://www.wolframalpha.com/input/?i=lcm%28{}%2C+{}%2C+{}%29", x_cycle_len, y_cycle_len, z_cycle_len);

		assert_eq!((x_cycle_len, y_cycle_len, z_cycle_len), (268296, 231614, 108344));
	}

	Ok(())
}

#[derive(Clone, Debug, PartialEq)]
struct State {
	r: i64,
	v: i64,
}

fn step(states: &mut [State]) {
	for i in 0..states.len() {
		let r = states[i].r;
		let mut v = states[i].v;
		v += states.iter().map(|state| (state.r - r).signum()).sum::<i64>();
		states[i].v = v;
	}

	for state in states.iter_mut() {
		state.r += state.v;
	}
}

fn total_energy(states_x: &[State], states_y: &[State], states_z: &[State]) -> i64 {
	states_x.iter()
	.zip(states_y)
	.zip(states_z)
	.map(|((x, y), z)| (x.r.abs() + y.r.abs() + z.r.abs()) * (x.v.abs() + y.v.abs() + z.v.abs()))
	.sum()
}

fn find_cycle_len(states_a: &mut [State], states_b: &mut [State]) -> usize {
	// Tortoise-and-hare cycle detection

	loop {
		step(states_a);
		step(states_b);
		step(states_b);

		if states_a == states_b {
			break;
		}
	}


	// states_a and states_b are identical now. Now keep stepping states_b forward till it becomes equal again.

	let mut result = 0;

	loop {
		step(states_b);

		result += 1;

		if states_a == states_b {
			break;
		}
	}

	result
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_total_energy() {
		fn test(
			r_x: &[i64],
			r_y: &[i64],
			r_z: &[i64],
			num_steps: usize,
			expected_total_energy: i64,
		) {
			let mut states_x: Vec<_> = r_x.iter().map(|&r| super::State { r, v: 0 }).collect();
			let mut states_y: Vec<_> = r_y.iter().map(|&r| super::State { r, v: 0 }).collect();
			let mut states_z: Vec<_> = r_z.iter().map(|&r| super::State { r, v: 0 }).collect();

			for _ in 0..num_steps {
				super::step(&mut states_x);
				super::step(&mut states_y);
				super::step(&mut states_z);
			}

			let actual_total_energy = super::total_energy(&states_x, &states_y, &states_z);
			assert_eq!(expected_total_energy, actual_total_energy);
		}

		test(
			&[-1, 2, 4, 3],
			&[0, -10, -8, 5],
			&[2, -7, 8, -1],
			10,
			179,
		);

		test(
			&[-8, 5, 2, 9],
			&[-10, 5, -7, -8],
			&[0, 10, 3, -3],
			100,
			1940,
		);
	}

	#[test]
	fn test_find_cycle_len() {
		fn test(
			r_x: &[i64],
			r_y: &[i64],
			r_z: &[i64],
			expected_cycle_len: (usize, usize, usize),
		) {
			let states_x: Vec<_> = r_x.iter().map(|&r| super::State { r, v: 0 }).collect();
			let states_y: Vec<_> = r_y.iter().map(|&r| super::State { r, v: 0 }).collect();
			let states_z: Vec<_> = r_z.iter().map(|&r| super::State { r, v: 0 }).collect();

			let mut states_a = (states_x, states_y, states_z);
			let mut states_b = states_a.clone();

			let x_cycle_len = super::find_cycle_len(&mut states_a.0, &mut states_b.0);
			let y_cycle_len = super::find_cycle_len(&mut states_a.1, &mut states_b.1);
			let z_cycle_len = super::find_cycle_len(&mut states_a.2, &mut states_b.2);
			let actual_cycle_len = (x_cycle_len, y_cycle_len, z_cycle_len);
			assert_eq!(expected_cycle_len, actual_cycle_len);
		}

		test(
			&[-1, 2, 4, 3],
			&[0, -10, -8, 5],
			&[2, -7, 8, -1],
			(18, 28, 44),
		);

		test(
			&[-8, 5, 2, 9],
			&[-10, 5, -7, -8],
			&[0, 10, 3, -3],
			(2028, 5898, 4702),
		);
	}
}
