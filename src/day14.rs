pub(super) fn run() -> Result<(), super::Error> {
	let mut input = vec![];
	for line in super::read_input_lines::<String>("day14")? {
		input.push(line?);
	}
	let reactions = parse(&input)?;

	{
		let result = get_required_ore(&reactions, 1)?;

		println!("14a: {}", result);

		assert_eq!(result, 843220);
	};

	{
		let result = fuel_from_trillion_ore(&reactions)?;

		println!("14b: {}", result);

		assert_eq!(result, 2169535);
	}

	Ok(())
}

/// This function returns a map of reactions.
///
/// The key is the product. The value is a tuple of the number of product generated by the reaction, and a map of ingredients.
///
/// Each entry in the map of ingredients is the name of the ingredient and the number of that ingredient used in the reaction.
fn parse(input: &[String]) -> Result<std::collections::BTreeMap<&str, (u64, Vec<(&str, u64)>)>, super::Error> {
	input.iter()
		.map(|line| {
			let mut sides = line.split(" => ");
			let lhs = sides.next().expect("str::split always has at least one part");
			let rhs = sides.next().ok_or("malformed input")?;

			let ingredients: Result<_, super::Error> =
				lhs.split(", ").map(|part| {
					let mut parts = part.split(' ');
					let amount = parts.next().expect("str::split always has at least one part").parse()?;
					let name = parts.next().ok_or("malformed input")?;
					Ok((name, amount))
				})
				.collect();
			let ingredients = ingredients?;

			let mut product_parts = rhs.split(' ');
			let product_num = product_parts.next().expect("str::split always has at least one part").parse()?;
			let product_name = product_parts.next().ok_or("malformed input")?;
			Ok((product_name, (product_num, ingredients)))
		})
		.collect()
}

fn get_required_ore<'a>(
	reactions: &'a std::collections::BTreeMap<&'a str, (u64, Vec<(&'a str, u64)>)>,
	fuel: u64,
) -> Result<u64, super::Error> {
	let mut ore = 0;

	let mut need: std::collections::BTreeMap<&str, u64> = Default::default();
	need.insert("FUEL", fuel);

	let mut workspace: std::collections::BTreeMap<&'a str, u64> = Default::default();

	loop {
		let (product, mut product_num) = {
			if let Some(&first) = need.keys().next() {
				(first, need.remove(&first).unwrap())
			}
			else {
				break;
			}
		};

		// First remove what we need from the workspace
		{
			let already_have = workspace.entry(product).or_default();
			let can_use_what_already_have = std::cmp::min(*already_have, product_num);
			*already_have -= can_use_what_already_have;
			product_num -= can_use_what_already_have;
		}

		if product_num == 0 {
			// Got everything we need from the workspace
			continue;
		}

		// Ore isn't explicitly listed in the workspace. Just pretend we have however much we need.
		if product == "ORE" {
			ore += product_num;
			continue;
		}

		// Going to find the reaction for this now. First restore the need for the current product.
		*need.entry(product).or_default() += product_num;

		let reaction = reactions.get(product).ok_or_else(|| format!("no reaction produced {}", product))?;

		let num_reactions = (product_num + reaction.0 - 1) / reaction.0; // Round up

		// Execute the reaction num_reaction times. We now have some product to add to the workspace...
		*workspace.entry(product).or_default() += reaction.0 * num_reactions;

		// ... and the ingredients for each reaction to add to the list of needed reactants.
		for (ingredient, ingredient_num) in &reaction.1 {
			*need.entry(ingredient).or_default() += *ingredient_num * num_reactions;
		}
	}

	Ok(ore)
}

fn fuel_from_trillion_ore(reactions: &std::collections::BTreeMap<&str, (u64, Vec<(&str, u64)>)>) -> Result<u64, super::Error> {
	// Find some power of two for which more than one trillion ore is used,
	// then do binary search to find the answer.

	let mut low = 1;
	let mut high = 1;

	loop {
		let result = get_required_ore(&reactions, high)?;
		match result.cmp(&1_000_000_000_000) {
			std::cmp::Ordering::Less => {
				low = high;
				high *= 2;
			},

			// Lucky
			std::cmp::Ordering::Equal => return Ok(high),

			std::cmp::Ordering::Greater => break,
		}
	}

	while high - low > 1 {
		let mid = (low + high) / 2;
		let result = get_required_ore(&reactions, mid)?;
		match result.cmp(&1_000_000_000_000) {
			std::cmp::Ordering::Less => low = mid,

			// Lucky
			std::cmp::Ordering::Equal => return Ok(mid),

			std::cmp::Ordering::Greater => high = mid,
		}
	}

	Ok(low)
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_get_required_ore() {
		fn test(
			input: &[&str],
			expected_ore: u64,
		) {
			let input: Vec<_> = input.iter().copied().map(ToOwned::to_owned).collect();
			let reactions = super::parse(&input).unwrap();

			let actual_ore = super::get_required_ore(&reactions, 1).unwrap();
			assert_eq!(expected_ore, actual_ore);
		}

		test(&[
			"10 ORE => 10 A",
			"1 ORE => 1 B",
			"7 A, 1 B => 1 C",
			"7 A, 1 C => 1 D",
			"7 A, 1 D => 1 E",
			"7 A, 1 E => 1 FUEL",
		], 31);

		test(&[
			"9 ORE => 2 A",
			"8 ORE => 3 B",
			"7 ORE => 5 C",
			"3 A, 4 B => 1 AB",
			"5 B, 7 C => 1 BC",
			"4 C, 1 A => 1 CA",
			"2 AB, 3 BC, 4 CA => 1 FUEL",
		], 165);

		test(&[
			"157 ORE => 5 NZVS",
			"165 ORE => 6 DCFZ",
			"44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
			"12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
			"179 ORE => 7 PSHF",
			"177 ORE => 5 HKGWZ",
			"7 DCFZ, 7 PSHF => 2 XJWVT",
			"165 ORE => 2 GPVTF",
			"3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
		], 13312);

		test(&[
			"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
			"17 NVRVD, 3 JNWZP => 8 VPVL",
			"53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
			"22 VJHF, 37 MNCFX => 5 FWMGM",
			"139 ORE => 4 NVRVD",
			"144 ORE => 7 JNWZP",
			"5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
			"5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
			"145 ORE => 6 MNCFX",
			"1 NVRVD => 8 CXFTF",
			"1 VJHF, 6 MNCFX => 4 RFSQX",
			"176 ORE => 6 VJHF",
		], 180697);

		test(&[
			"171 ORE => 8 CNZTR",
			"7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
			"114 ORE => 4 BHXH",
			"14 VRPVC => 6 BMBT",
			"6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
			"6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
			"15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
			"13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
			"5 BMBT => 4 WPTQ",
			"189 ORE => 9 KTJDG",
			"1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
			"12 VRPVC, 27 CNZTR => 2 XDBXC",
			"15 KTJDG, 12 BHXH => 5 XCVML",
			"3 BHXH, 2 VRPVC => 7 MZWV",
			"121 ORE => 7 VRPVC",
			"7 XCVML => 6 RJRHP",
			"5 BHXH, 4 VRPVC => 5 LTCX",
		], 2210736);
	}

	#[test]
	fn test_fuel_from_trillion_ore() {
		fn test(
			input: &[&str],
			expected_fuel: u64,
		) {
			let input: Vec<_> = input.iter().copied().map(ToOwned::to_owned).collect();
			let reactions = super::parse(&input).unwrap();

			let actual_fuel = super::fuel_from_trillion_ore(&reactions).unwrap();
			assert_eq!(expected_fuel, actual_fuel);
		}

		test(&[
			"157 ORE => 5 NZVS",
			"165 ORE => 6 DCFZ",
			"44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
			"12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
			"179 ORE => 7 PSHF",
			"177 ORE => 5 HKGWZ",
			"7 DCFZ, 7 PSHF => 2 XJWVT",
			"165 ORE => 2 GPVTF",
			"3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
		], 82892753);

		test(&[
			"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
			"17 NVRVD, 3 JNWZP => 8 VPVL",
			"53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
			"22 VJHF, 37 MNCFX => 5 FWMGM",
			"139 ORE => 4 NVRVD",
			"144 ORE => 7 JNWZP",
			"5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
			"5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
			"145 ORE => 6 MNCFX",
			"1 NVRVD => 8 CXFTF",
			"1 VJHF, 6 MNCFX => 4 RFSQX",
			"176 ORE => 6 VJHF",
		], 5586022);

		test(&[
			"171 ORE => 8 CNZTR",
			"7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
			"114 ORE => 4 BHXH",
			"14 VRPVC => 6 BMBT",
			"6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
			"6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
			"15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
			"13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
			"5 BMBT => 4 WPTQ",
			"189 ORE => 9 KTJDG",
			"1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
			"12 VRPVC, 27 CNZTR => 2 XDBXC",
			"15 KTJDG, 12 BHXH => 5 XCVML",
			"3 BHXH, 2 VRPVC => 7 MZWV",
			"121 ORE => 7 VRPVC",
			"7 XCVML => 6 RJRHP",
			"5 BHXH, 4 VRPVC => 5 LTCX",
		], 460664);
	}
}