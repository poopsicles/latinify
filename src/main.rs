fn is_vowel(ch: &char) -> bool {
	match ch.to_ascii_lowercase() {
		'a'|'e'|'i'|'o'|'u' => true,
		_ => false
	}
}

fn first_vowel(s: &str) -> usize {
	let mut pos = 0usize;
	for (position, letter) in s.chars().enumerate() {
		if is_vowel(&letter) {
			pos = position;
			break;
		}
	}

	pos
}

fn main() {
	let input = String::from("The fox bit me and ate");
	let mut out = String::new();

	// split the input string into words
	for word in input.split_whitespace() {
		let latin_word = match is_vowel(&word.chars().nth(0).unwrap()) { // check the first letter
			true => format!("{}-hay", word), // if it's a vowel, just append "hay"
			false => { // if a consonant, correct capitals, move to end, and append "ay"
				let x = first_vowel(&word); // get the position of the first consonant cluster
				match word.chars().nth(0).unwrap().is_uppercase() { // checking capitalisation
					true => format!("{}{}-{}ay", &word[x..x + 1].to_uppercase(), &word[x + 1..], &word[0..x].to_lowercase()),
					false => format!("{}-{}ay", &word[x..], &word[0..x]) 
				}
			}
		};
		
		out = format!("{} {}", out, latin_word); // concatenate
	}
	let out = out.trim().to_string();
	println!("|{}|", out);
}

// split string into words
// if first letter is vowel, add "hay"
// else get position of first vowel (consonant cluster slice) and move to end and add "ay"
// correct capitalisation
// append
