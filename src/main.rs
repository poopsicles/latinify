use std::io::{self, Write};
use rayon::prelude::*;
use latinify::{latinifier, tokenise};

fn main() {
    println!("Welcome to latinify! Type a sentence and press Return/Enter to get started...");
    let mut text = String::new();

    // main program loop:
    // collect input, check if it's a command, and print the result
    loop {
        text.clear(); // remove any previous input

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut text).unwrap();

        match text.trim().to_ascii_lowercase().as_str() { // possible commands
            "" => {
                println!("Type \":exit\" to quit, or \":help\" for help\n");
                continue;
            }

            ":exit" => {
                println!("Goodbye...");
                break;
            }

            ":help" => {
                println!("Type a sentence, then press Return/Enter to get the translation...");
                println!("Type \":exit\" to quit\n");
                continue;
            }

            _ => (),
        }

        println!( // latinify words and not contiguous characters
            "{}",
            tokenise(&text)
                .into_par_iter()
                .map(|x| match x {
                    latinify::Sequence::Characters(s) => s,
                    latinify::Sequence::Word(s) => latinifier(&s),
                })
                .collect::<String>()
        );
    }
}
