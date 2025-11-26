use std::collections::HashMap;
use std::io;
use clap::{Arg, Command, ArgAction};
fn compte(phrase:String){
    let phrase = phrase.to_lowercase();
    let mut compteur = HashMap::new();
    for mot in phrase.split_whitespace() {
        *compteur.entry(mot.to_string()).or_insert(0) += 1;
    }
    println!("Occurrences des mots :");
    for (mot, nb) in compteur {
        println!("{} : {}", mot, nb);
    }
}
fn main() {
    let matches = Command::new("compteur_app")
        .version("1.0")
        .author("Olivier")
        .about("Compte le nombre d'itérations des mots d'une phrase")
        .arg(
            Arg::new("phrase")
            .help("Phrase à étudier")
            .required(true)
            .index(1),
        )
        .get_matches();
        let phrase=matches
            .get_one::<String>("phrase")
            .unwrap()
            .clone();
        compte(phrase);
}
