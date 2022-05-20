use latinify::{latinifier, tokenise};
use rayon::prelude::*;
use std::io::{self, Write};

fn main() {
    println!("Welcome to latinify! Type a sentence and press Enter to get started...");
    let mut text = String::new();

    loop {
        text.clear();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut text).unwrap();

        match text.trim().to_ascii_lowercase().as_str() {
            "" => {
                println!("Type \":exit\" to quit, or \":help\" for help\n");
                continue;
            }

            ":exit" => {
                println!("Goodbye...");
                break;
            }

            ":help" => {
                println!("Type a sentence, then press Enter/Return to get the translation");
                continue;
            }

            _ => (),
        }

        println!(
            "{}",
            tokenise(&text)
                .into_par_iter()
                .map(|x| match x {
                    latinify::Kind::Characters(s) => s.to_string(),
                    latinify::Kind::Word(s) => latinifier(&s),
                })
                .collect::<String>()
        );
    }
}
