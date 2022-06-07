pub(super) fn run() -> Result<(), super::Error> {
	let line = super::read_input_lines::<String>("day16")?.next().ok_or("file is empty")??;
	let input = line.chars().map(|c| (c as u8 - b'0').into());

	{
		let result = part1(input.clone(), 100);

		println!("16a: {result}");

		assert_eq!(result, 68317988);
	}

	{
		let input: Vec<_> = input.collect();

		let result = part2(&input)?;

		println!("16b: {result}");

		assert_eq!(result, 53850800);
	}

	Ok(())
}

fn run_phase(input: &mut [i32], output: &mut [i32], start_at: usize) {
	assert_eq!(input.len(), output.len());

	output[0] = 0;

	if start_at > input.len() / 2 {
		// Optimization: When start_at is greater than half of the input, then all cells from start_end to the end of the input
		// are in the "1" part of the pattern. So every output cell at position N has a value that is the sum of all the input cells from N to the end of the input.
		//
		// Also, we can avoid re-computing the same sums over and over again by calculating the sums backwards and reusing the previously calculated one.
		//
		// Note: There is a more general optimization: When start_at is greater than a third of the input, then all cells from start_end to the end of the input
		// are in the "1" part of the pattern or the second "0" part of the pattern. So every output cell at position N has a value that is
		// the sum of all the input cells from N to 2*N. But in this case we lose the second optimization of calculating the sums from the right and reusing them.
		//
		// Since the puzzle only sets start_at to greater than half of the input, this more general optimization has not been implemented.
		let _ = output.iter_mut().zip(input.iter()).skip(start_at).rev().fold(0, |previous_sum, (output, input)| {
			*output = (previous_sum + *input).abs() % 10;
			previous_sum + *input
		});
	}
	else {
		output.iter_mut().skip(1).enumerate().skip(start_at).for_each(|(i, output)| {
			let output_element: i32 =
				input
				.chunks(i + 1)
				.enumerate()
				.fold(0, |sum, (chunk_num, chunk)| match chunk_num % 4 {
					0 | 2 => sum,
					1 => sum + chunk.iter().sum::<i32>(),
					3 => sum - chunk.iter().sum::<i32>(),
					_ => unreachable!(),
				});
			*output = output_element.abs() % 10;
		});
	}
}

fn num_from_digits(digits: &[i32]) -> i32 {
	digits.iter().copied().fold(0, |prev, curr| prev * 10 + curr)
}

fn part1(input: impl Iterator<Item = i32>, num_phases: usize) -> i32 {
	// The expanded pattern skips its first element, so prepend a 0 to the input so that its chunks line up with the pattern
	let mut input: Vec<_> = std::iter::once(0).chain(input).collect();
	let mut output = vec![0; input.len()];
	for _ in 0..num_phases {
		run_phase(&mut input, &mut output, 0);
		std::mem::swap(&mut input, &mut output);
	}

	num_from_digits(&input[1..9])
}

fn part2(input: &[i32]) -> Result<i32, super::Error> {
	let digest_pos: usize = std::convert::TryInto::try_into(num_from_digits(&input[0..7]))?;

	let input_len = input.len() * 10_000;
	// The expanded pattern skips its first element, so prepend a 0 to the input so that its chunks line up with the pattern
	let mut input: Vec<_> = std::iter::once(0).chain(input.iter().copied().cycle().take(input_len)).collect();

	let mut output = vec![0; input.len()];
	for _ in 0..100 {
		// Input elements before position N do not contribute to output elements at position N and higher.
		// So it's safe to skip calculating any cells before the digest_pos'th position.
		run_phase(&mut input, &mut output, digest_pos);
		std::mem::swap(&mut input, &mut output);
	}

	let result = num_from_digits(&input[(digest_pos + 1)..][..8]);
	Ok(result)
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_part1() {
		fn test(input: &str, num_phases: usize, expected_output: i32) {
			let input = input.chars().map(|c| (u8::try_from(c).unwrap() - b'0').into());
			let actual_output = super::part1(input, num_phases);
			assert_eq!(expected_output, actual_output);
		}

		test("12345678", 1, 48226158);
		test("12345678", 2, 34040438);
		test("12345678", 3, 03415518);
		test("12345678", 4, 01029498);

		test("80871224585914546619083218645595", 100, 24176176);
		test("19617804207202209144916044189917", 100, 73745418);
		test("69317163492948606335995924319873", 100, 52432133);
	}

	#[test]
	fn test_part2() {
		fn test(input: &str, expected_digest: i32) {
			let input: Vec<_> = input.chars().map(|c| (u8::try_from(c).unwrap() - b'0').into()).collect();
			let actual_digest = super::part2(&input).unwrap();
			assert_eq!(expected_digest, actual_digest);
		}

		test("03036732577212944063491565474664", 84462026);
		test("02935109699940807407585447034323", 78725270);
		test("03081770884921959731165446850517", 53553731);
	}
}
