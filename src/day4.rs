pub(super) fn run() -> Result<(), super::Error> {
	let input = super::read_input_lines::<String>("day4")?.next().ok_or("file is empty")??;
	let mut input = input.split('-');
	let lower: u32 = input.next().ok_or("malformed input")?.parse()?;
	let upper: u32 = input.next().ok_or("malformed input")?.parse()?;

	{
		let result = (lower..=upper).filter(|&num| is_valid(num)).count();

		println!("4a: {result}");

		assert_eq!(result, 1919);
	}

	{
		let result = (lower..=upper).filter(|&num| is_valid2(num)).count();

		println!("4b: {result}");

		assert_eq!(result, 1291);
	}

	Ok(())
}

#[allow(clippy::cast_possible_truncation)]
fn digits(mut num: u32) -> Vec<u8> {
	let mut result = vec![];
	while num > 0 {
		result.push(u8::try_from(num % 10).expect("0..10 fits in u8"));
		num /= 10;
	}
	result
}

fn histogram(digits: &[u8]) -> [usize; 10] {
	let mut result = [0; 10];
	for &digit in digits {
		result[usize::from(digit)] += 1;
	}
	result
}

fn is_valid(num: u32) -> bool {
	let digits = digits(num);
	let histogram = histogram(&digits);

	let adjacent_same = histogram.iter().any(|&count| count >= 2);
	let monotonic_increasing = digits.windows(2).all(|pair| pair[0] >= pair[1]);
	adjacent_same && monotonic_increasing
}

fn is_valid2(num: u32) -> bool {
	let digits = digits(num);
	let histogram = histogram(&digits);

	let adjacent_same_and_not_part_of_larger_group = histogram.iter().any(|&count| count == 2);
	let monotonic_increasing = digits.windows(2).all(|pair| pair[0] >= pair[1]);
	adjacent_same_and_not_part_of_larger_group && monotonic_increasing
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_is_valid() {
		assert!(super::is_valid(111111));
		assert!(!super::is_valid(223450));
		assert!(!super::is_valid(123789));
	}

	#[test]
	fn test_is_valid2() {
		assert!(super::is_valid2(112233));
		assert!(!super::is_valid2(123444));
		assert!(super::is_valid2(111122));
	}
}
