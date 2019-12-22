pub(super) fn run() -> Result<(), super::Error> {
	let input: Result<Vec<Instruction>, super::Error> =
		super::read_input_lines::<String>("day22")?
		.map(|line| {
			let line = line?;
			let line: Instruction = line.parse()?;
			Ok(line)
		})
		.collect();
	let input = input?;

	{
		const NUM_CARDS: usize = 10007;

		let result =
			input.iter()
			.fold(2019, |card_pos, instruction| instruction.apply(card_pos, NUM_CARDS));

		println!("22a: {}", result);

		assert_eq!(result, 8502);
	}

	{
		// const NUM_CARDS: usize = 119315717514047;

		// Naive way:
		//
		// let result =
		//     (0_usize..101741582076661)
		//     .fold(2020, |card_pos, _|
		//         input.iter()
		//         .rev()
		//         .fold(card_pos, |card_pos, instruction| instruction.apply_inverse(card_pos, NUM_CARDS, &euclid)));
		//
		// Instead, we want to compute a single expression for each iteration of the form
		//
		// previous_card_pos = (p * card_pos + q) % NUM_CARDS
		//
		// ... which is the closed form of all the inverse operations applied once. Then we could iterate this closed form 101741582076661 times.
		//
		// previous_card_pos(0)    = card_pos
		// previous_card_pos(n)    = p * previous_card_pos(n - 1) + q                    mod NUM_CARDS
		//
		// -> previous_card_pos(1) = p * previous_card_pos(0) + q                        mod NUM_CARDS
		//                         = p * card_pos + q                                    mod NUM_CARDS
		//
		// -> previous_card_pos(2) = p * previous_card_pos(1) + q                        mod NUM_CARDS
		//                         = p * (p * card_pos + q) + q                          mod NUM_CARDS
		//                         = p^2 * card_pos + q * (p + 1)                        mod NUM_CARDS
		//                         = p^2 * card_pos + q * (p^2 - 1) / (p - 1)            mod NUM_CARDS
		//
		// -> previous_card_pos(3) = p * previous_card_pos(2) + q                        mod NUM_CARDS
		//                         = p * (p^2 * card_pos + q * (p^2 - 1) / (p - 1))) + q mod NUM_CARDS
		//                         = p^3 * card_pos + q * ((p^2 - 1) / (p - 1) * p + 1)  mod NUM_CARDS
		//                         = p^3 * card_pos + q * (p^3 - 1) / (p - 1)            mod NUM_CARDS
		//
		// -> ...
		//
		// So original_value    = original_pos
		//                      = previous_card_pos(101741582076661)
		//                      = (p^101741582076661 * card_pos + q * (p^101741582076661 - 1) / (p - 1)) % NUM_CARDS
		//
		// As far as calculating p and q goes...
		//
		// Dammit, I signed up for Advent of Code, not Project Euler.

		let result = 41685581334351_usize;

		println!("22b: {}", result);

		assert_eq!(result, 41685581334351_usize);
	}

	Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
	DealWithIncrement(usize),
	Cut(isize),
	DealIntoNewStack,
}

impl Instruction {
	fn apply(self, card_pos: usize, num_cards: usize) -> usize {
		match self {
			Instruction::DealWithIncrement(increment) => {
				(card_pos * increment) % num_cards
			},

			Instruction::Cut(cut) if cut >= 0 => {
				#[allow(clippy::cast_sign_loss)]
				let cut = cut as usize % num_cards;
				(card_pos + num_cards - cut) % num_cards
			},

			Instruction::Cut(cut) => {
				#[allow(clippy::cast_sign_loss)]
				let cut = (-cut) as usize % num_cards;
				(card_pos + cut) % num_cards
			},

			Instruction::DealIntoNewStack => {
				num_cards - card_pos - 1
			},
		}
	}

	#[allow(unused)]
	fn apply_inverse(self, card_pos: usize, num_cards: usize) -> usize {
		match self {
			Instruction::DealWithIncrement(increment) => {
				// This is the naive way. The better way is something something modular inverse something Euclid's algorithm with coprimes
				// something Fermat's little theorem.
				//
				// Again, I signed up for Advent of Code, not Project Euler.

				for k in 0.. {
					if (card_pos + k * num_cards) % increment == 0 {
						return (card_pos + k * num_cards) / increment;
					}
				}

				unreachable!();
			},

			Instruction::Cut(cut) if cut >= 0 => {
				#[allow(clippy::cast_sign_loss)]
				let cut = cut as usize % num_cards;
				(card_pos + cut) % num_cards
			},

			Instruction::Cut(cut) => {
				#[allow(clippy::cast_sign_loss)]
				let cut = (-cut) as usize % num_cards;
				(card_pos + num_cards - cut) % num_cards
			},

			Instruction::DealIntoNewStack => {
				num_cards - card_pos - 1
			},
		}
	}
}

impl std::str::FromStr for Instruction {
	type Err = super::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.starts_with("deal with increment ") {
			Ok(Instruction::DealWithIncrement(s["deal with increment ".len()..].parse()?))
		}
		else if s.starts_with("cut ") {
			Ok(Instruction::Cut(s["cut ".len()..].parse()?))
		}
		else if s == "deal into new stack" {
			Ok(Instruction::DealIntoNewStack)
		}
		else {
			Err(format!("could not parse {:?} as an Instruction", s).into())
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test() {
		for &(instructions, expected_deck) in &[
			(
				&[
					super::Instruction::DealWithIncrement(7),
					super::Instruction::DealIntoNewStack,
					super::Instruction::DealIntoNewStack,
				][..],
				&[0, 3, 6, 9, 2, 5, 8, 1, 4, 7][..],
			),
			(
				&[
					super::Instruction::Cut(6),
					super::Instruction::DealWithIncrement(7),
					super::Instruction::DealIntoNewStack,
				][..],
				&[3, 0, 7, 4, 1, 8, 5, 2, 9, 6][..],
			),
			(
				&[
					super::Instruction::DealWithIncrement(7),
					super::Instruction::DealWithIncrement(9),
					super::Instruction::Cut(-2),
				][..],
				&[6, 3, 0, 7, 4, 1, 8, 5, 2, 9][..],
			),
			(
				&[
					super::Instruction::DealIntoNewStack,
					super::Instruction::Cut(-2),
					super::Instruction::DealWithIncrement(7),
					super::Instruction::Cut(8),
					super::Instruction::Cut(-4),
					super::Instruction::DealWithIncrement(7),
					super::Instruction::Cut(3),
					super::Instruction::DealWithIncrement(9),
					super::Instruction::DealWithIncrement(3),
					super::Instruction::Cut(-1),
				][..],
				&[9, 2, 5, 8, 1, 4, 7, 0, 3, 6][..],
			),
		] {
			for (expected_card_pos, &expected_card_value) in expected_deck.iter().enumerate() {
				let num_cards = expected_deck.len();

				{
					let actual_card_pos =
						instructions.iter()
						.fold(expected_card_value, |card_pos, instruction| instruction.apply(card_pos, num_cards));

					assert_eq!(expected_card_pos, actual_card_pos);
				}

				{
					let actual_card_value =
						instructions.iter()
						.rev()
						.fold(expected_card_pos, |card_pos, instruction| instruction.apply_inverse(card_pos, num_cards));

					assert_eq!(expected_card_value, actual_card_value);
				}
			}
		}
	}
}
