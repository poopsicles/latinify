#[inline]
fn is_vowel(ch: &char) -> bool {
    // checks if a character is a vowel
    match ch.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

#[derive(Debug)]
pub enum Kind { // used to differentiate between actual words and contiguous characters
    Word(String),
    Characters(String),
}

impl Kind {
    fn push(&mut self, ch: &char) { // adds a character to the inner value
        match self {
            Kind::Characters(s) => s.push(*ch),
            Kind::Word(s) => s.push(*ch),
        }
    }
}

pub fn tokenise(s: &str) -> Vec<Kind> { // tokenises a string into words and characters
    let mut is_prev_char = false; // for storing whether the previous character was a character
    let mut res: Vec<Kind> = vec![];

    for ch in s.chars() {
        match (ch.is_alphabetic(), is_prev_char) {
            (true, false) => {
                res.push(Kind::Word(ch.to_string()));
                is_prev_char = true;
            }

            (false, true) => {
                res.push(Kind::Characters(ch.to_string()));
                is_prev_char = false;
            }

            (_, _) => {
                let mut previous = res.pop().unwrap_or(Kind::Characters(String::new()));
                previous.push(&ch);
                res.push(previous);
            }
        }
    }

    res
}

pub fn latinifier(s: &str) -> String { // returns the Pig Latin version of a word
    let len = s.len();
    let mut vow = len;

	let first = s.chars().next().unwrap(); 
	let second = s.chars().nth(1).unwrap_or(' '); 

    for (position, letter) in s.char_indices() {
        // gets the index of the first vowel
        if is_vowel(&letter) {
            vow = position;
            break;
        }
    }

	if vow == len { // there's no vowel in the word
		match (first.is_uppercase(), second.is_uppercase()){
			(true, true) => return format!("{}AY", s),
			(_, _) => return format!("{}ay", s),
		}
	}

	if vow == 0 { // the word starts with a vowel
		match (first.is_uppercase(), second.is_uppercase()){
			(true, true) => return format!("{}HAY", s),
			(_, _) => return format!("{}hay", s),
		}
	}
	
	let (cluster, segment) = s.split_at(vow); // splits the word into the consonant cluster and the non-moving segment
	match (first.is_uppercase(), second.is_uppercase()){
		(true, true) => return format!("{}{}AY", segment, cluster),
		(true, false) => return format!("{}{}{}ay", &segment[0..1].to_ascii_uppercase(), &segment[1..], cluster.to_ascii_lowercase()),
		(false, _) => return format!("{}ay", s),
	}
}
