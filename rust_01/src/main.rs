use clap::Parser;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    author = "Olivier",
    about = "Compte le nombre d'itérations d'un mot dans une phrase"
)]
struct Args {
    #[arg(index=1,default_value="")]
    phrase: String,

    #[arg(long,short='n',default_value_t = 0)]
    top: usize,

    #[arg(long = "min",short,default_value_t = 1)]
    min_length: usize,

    #[arg(long,short)]
    ignore_case: bool,
}

fn main() {
    let mut args = Args::parse();
    if args.phrase.is_empty(){
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        args.phrase = buffer.trim().to_string();
    }
    compte_top(args.phrase, args.top, args.min_length, args.ignore_case);
}

fn compte_top(phrase: String, top: usize, min: usize, ignore: bool) {
    let phrase_processed = if ignore {
        phrase.to_lowercase()
    } else {
        phrase
    };

    let mut compteur = HashMap::new();

    for mot in phrase_processed.split_whitespace() {
        if mot.len() >= min {
            *compteur.entry(mot.to_string()).or_insert(0) += 1;
        }
    }

    let mut classement: Vec<(String, u32)> = compteur.into_iter().collect();
    classement.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Occurrences des mots :");
    let iter = classement.iter();
    if top != 0 {
        for (mot, nb) in iter.take(top) {
            println!("{}: {}", mot, nb);
        }
    } else {
        for (mot, nb) in iter {
            println!("{}: {}", mot, nb);
        }
    }
}
