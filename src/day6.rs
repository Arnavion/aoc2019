pub(super) fn run() -> Result<(), super::Error> {
	let input: Result<Vec<String>, super::Error> = super::read_input_lines::<String>("day6")?.collect();
	let input = input?;

	let mut orbits_parent_to_child: std::collections::BTreeMap<&str, Vec<&str>> = Default::default();
	let mut orbits_child_to_parent: std::collections::BTreeMap<&str, &str> = Default::default();

	for line in &input {
		let mut parts = line.split(')');
		let parent = parts.next().expect("str::split always has at least one part");
		let child = parts.next().ok_or("malformed input")?;

		orbits_parent_to_child.entry(parent).or_default().push(child);
		orbits_child_to_parent.insert(child, parent);
	}

	{
		let result = num_orbits(&orbits_parent_to_child);

		println!("6a: {result}");

		assert_eq!(result, 144909);
	}

	{
		let result = num_transfers("YOU", "SAN", &orbits_child_to_parent)?;

		println!("6b: {result}");

		assert_eq!(result, 259);
	}

	Ok(())
}

fn num_orbits(orbits_parent_to_child: &std::collections::BTreeMap<&str, Vec<&str>>) -> usize {
	let mut result = 0;

	let mut nodes: std::collections::VecDeque<(&str, usize)> = vec![("COM", 0)].into_iter().collect();

	while let Some((node, depth)) = nodes.pop_front() {
		result += depth;
		nodes.extend(
			orbits_parent_to_child.get(node)
			.map(std::ops::Deref::deref)
			.unwrap_or_default()
			.iter()
			.map(|&child| (child, depth + 1))
		);
	}

	result
}

fn num_transfers(start_child: &str, end_child: &str, orbits_child_to_parent: &std::collections::BTreeMap<&str, &str>) -> Result<usize, super::Error> {
	let path_from_start_child_to_com = path_from_body_to_com(start_child, orbits_child_to_parent);
	let path_from_end_child_to_com = path_from_body_to_com(end_child, orbits_child_to_parent);

	for (start_ancestor_pos, &start_ancestor) in path_from_start_child_to_com.iter().enumerate() {
		if let Some(end_ancestor_pos) = path_from_end_child_to_com.iter().position(|&end_ancestor| end_ancestor == start_ancestor) {
			return Ok(start_ancestor_pos + end_ancestor_pos);
		}
	}

	Err("no solution".into())
}

fn path_from_body_to_com<'a>(start_child: &'a str, orbits_child_to_parent: &std::collections::BTreeMap<&str, &'a str>) -> Vec<&'a str> {
	let mut result = vec![];

	let mut current = Some(start_child);
	while let Some(current_) = current {
		current = orbits_child_to_parent.get(current_).copied();
		if let Some(current) = current {
			result.push(current);
		}
	}

	result
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_num_orbits() {
		let orbits_parent_to_child =
			vec![
				("COM", vec!["B"]),
				("B", vec!["C", "G"]),
				("C", vec!["D"]),
				("D", vec!["E", "I"]),
				("E", vec!["F", "J"]),
				("G", vec!["H"]),
				("J", vec!["K"]),
				("K", vec!["L"]),
			]
			.into_iter()
			.collect();
		let num_orbits = super::num_orbits(&orbits_parent_to_child);
		assert_eq!(num_orbits, 42);
	}

	#[test]
	fn test_num_transfers() {
		let orbits_child_to_parent =
			vec![
				("B", "COM"),
				("C", "B"),
				("G", "B"),
				("D", "C"),
				("E", "D"),
				("I", "D"),
				("F", "E"),
				("J", "E"),
				("H", "G"),
				("K", "J"),
				("L", "K"),
				("YOU", "K"),
				("SAN", "I"),
			]
			.into_iter()
			.collect();

		let num_transfers = super::num_transfers("YOU", "SAN", &orbits_child_to_parent).unwrap();
		assert_eq!(num_transfers, 4);
	}
}
