fn is_vowel(ch: &char) -> bool {
	// checks if a character is a vowel

	match ch.to_ascii_lowercase() {
		'a'|'e'|'i'|'o'|'u' => true,
		_ => false
	}
}

fn latinify(s: &str) -> String { // returns the Pig Latin version of a word
	
	// handling solo non-alphanumeric characters
	if s.len() == 1 && !s.chars().nth(0).unwrap().is_alphanumeric() {
		return s.to_string();
	}
	
	// this gets the positions of the first vowel, first letter, and last letter in a string
	let mut vow = s.len();
	let mut first = 0usize;
	let mut last = 0usize;

	for (position, letter) in s.char_indices() { // gets the index of the first vowel
		if is_vowel(&letter) {
			vow = position;
			break;
		}
	}

	for (position, letter) in s.char_indices() { // gets the index of the first letter
		if letter.is_alphabetic() {
			first = position;
			break;
		}
	}

	for (position, letter) in s.char_indices().rev() { // gets the index of the last letter
		if letter.is_alphabetic() {
			last = position;
			break;
		}
	}

	// breaks a word down into its elements
	// e.g. (Run!) => "(", "R", "un", ")!"

	let before = &s[0..first]; // extranneous characters before any letter (punctuation)
	let cluster = &s[first..vow]; // the consonant cluster that is moved to the end of the word
	let segment = &s[vow..last + 1]; // the non-moving segment 
	let after = &s[last+ 1..]; // extranneous characters after (punctuation and whitespace)

	match is_vowel(&s.chars().nth(0).unwrap()) { // check the first letter of the word
		true => { // if it's a vowel, simply append "hay"
			if s.len() > 1 {
				if segment.chars().nth(1).unwrap().is_uppercase() { // checking capitalisation of the second letter of segment
					format!("{}{}-HAY{}", before, segment, after)
				} else {
					format!("{}{}-hay{}", before, segment, after)
				}
			} else {
				if segment.chars().nth(0).unwrap().is_uppercase() { // checking capitalisation of the first letter of segment
					format!("{}{}-HAY{}", before, segment, after)
				} else {
					format!("{}{}-hay{}", before, segment, after)
				}
			} 
		} 

		false => { // if not, move the consonant cluster to the end and append "ay"
			// checking capitalisation of the first letter of segment
			match segment.chars().nth(0) {
				Some(x) => {
					if x.is_uppercase() { // if it's an all-caps word
						format!("{}{}{}-AY{}", before, segment, cluster, after) 
					} else {
						if cluster.chars().nth(0).unwrap().is_uppercase() { // if it's a title word
							// correct capitalisation
							let segment = format!("{}{}", segment.chars().nth(0).unwrap().to_uppercase(), &segment[1..]);
							let cluster = cluster.to_lowercase();

							format!("{}{}{}-ay{}", before, segment, cluster, after)
						} else { // if it's a normal word
							format!("{}{}{}-ay{}", before, segment, cluster, after)
						}
					}
				}

				None => {
					if cluster.len() > 1 {
						if cluster.chars().nth(1).unwrap().is_uppercase() { // checking capitalisation of the second letter of segment
							format!("{}{}{}-AY{}", before, segment, cluster, after) 
						} else if cluster.chars().nth(0).unwrap().is_uppercase() { // if it's a title word
							// correct capitalisation
							let segment = format!("{}{}", segment.chars().nth(0).unwrap().to_uppercase(), &segment[1..]);
							let cluster = cluster.to_lowercase();

							format!("{}{}{}-ay{}", before, segment, cluster, after)
						} else {
							format!("{}{}{}-ay{}", before, segment, cluster, after)
						}
					} else {
						if cluster.chars().nth(0).unwrap().is_uppercase() { // if it's a title word
							// correct capitalisation
							let segment = format!("{}{}", segment.chars().nth(0).unwrap().to_uppercase(), &segment[1..]);
							let cluster = cluster.to_lowercase();

							format!("{}{}{}-ay{}", before, segment, cluster, after)
						} else {
							format!("{}{}{}-ay{}", before, segment, cluster, after)
						}
					} 
				}
			}	
		}
	}
}

fn main() {
	let input = String::from("\tIbadan,\n\trunning splash of rust\n\tand gold-flung and scattered\n\tamong seven hills like broken\n\tChina in the sun.");

	let mut out = String::new();

	// split the input string into words with any extranneous characters included
	// e.g "The brown fox!" => "The ", "brown ", "fox!"

	for word in input.split_inclusive(|c: char| !c.is_alphanumeric()) {		
		out = format!("{}{}", out, latinify(&word)); // concatenate the latinified version
	}
	println!("\n{}\n\t\t- J.P. Clark", input);
	println!("\nTranslated:\n{}", out);
}