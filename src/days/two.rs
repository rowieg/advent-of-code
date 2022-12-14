use std::collections::HashMap;
use crate::utils;


struct Scoretable<'a> {
	a: HashMap<&'a str, i32>,
	b: HashMap<&'a str, i32>,
	c: HashMap<&'a str, i32>,
}

fn calculate_score(enemy: &str, you: &str) -> i32 {

	let a_row = HashMap::from([
    ("X", 4),
    ("Y", 8),
    ("Z", 3),
	]);

	let b_row = HashMap::from([
    ("X", 1),
    ("Y", 5),
    ("Z", 9),
	]);

	let c_row = HashMap::from([
    ("X", 7),
    ("Y", 2),
    ("Z", 6),
	]);

	let score_table = Scoretable { a: a_row, b:b_row, c:c_row};

	if enemy.to_lowercase().eq("a") {
		return score_table.a[&you];
	}	else if enemy.to_lowercase().eq("b") {
		return score_table.b[&you];
	}
	else {
		return score_table.c[&you];
	}
}

fn calculate_sumerized_score(turns: &str) -> i32 {
	let mut sum = 0;
	for line in turns.lines() {
		let enemy = line.chars().next().unwrap().to_string();
		let you = line.chars().last().unwrap().to_string();
		sum += calculate_score(enemy.as_str(), you.as_str());
	}
	return sum;
}

pub fn get_answer() -> i32 {
	let content_as_str = utils::read_file_to_string("resources/two.txt");
	return calculate_sumerized_score(content_as_str.as_str());
}

#[cfg(test)]
mod tests {
	use super::{calculate_score, calculate_sumerized_score};

	#[test]
	fn calculate_score_test() {
			let actual_score = calculate_score("A", "Y");
			assert_eq!(8, actual_score);
		
			let actual_score = calculate_score("B", "X");
			assert_eq!(1, actual_score);

			let actual_score = calculate_score("C", "Z");
			assert_eq!(6, actual_score);
	}

	#[test]
	fn calculate_sumerized_score_test() {
		let test_data = "A Y\nB X\nC Z";
		let sumerized_score = calculate_sumerized_score(test_data);
		assert_eq!(15, sumerized_score);
	}
}