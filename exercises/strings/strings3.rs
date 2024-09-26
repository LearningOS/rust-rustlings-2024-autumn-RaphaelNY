// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
	// TODO: Remove whitespace from both ends of a string!
	// input.trim().to_string()
	let start = input.chars().position(|c| !c.is_whitespace()).unwrap();
	let end = input.chars().rev().position(|c| !c.is_whitespace()).unwrap();
	input[start..input.len() - end].to_string()
}

fn compose_me(input: &str) -> String {
	// TODO: Add " world!" to the string! There's multiple ways to do this!
	input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
	// TODO: Replace "cars" in the string with "balloons"!
	// input.replace("cars", "balloons")
	let target = "cars";
	let replacement = "balloons";

	let mut result = String::new();
	let mut start = 0;

	while let Some(pos) = input[start..].find(target) {
		result.push_str(&input[start..start + pos]);
		result.push_str(replacement);
		start += pos + target.len();
	}

	result.push_str(&input[start..]);
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn trim_a_string() {
		assert_eq!(trim_me("Hello!     "), "Hello!");
		assert_eq!(trim_me("  What's up!"), "What's up!");
		assert_eq!(trim_me("   Hola!  "), "Hola!");
	}

	#[test]
	fn compose_a_string() {
		assert_eq!(compose_me("Hello"), "Hello world!");
		assert_eq!(compose_me("Goodbye"), "Goodbye world!");
	}

	#[test]
	fn replace_a_string() {
		assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
		assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
	}
}
