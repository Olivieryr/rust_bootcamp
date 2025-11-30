use std::collections::HashMap;
use std::io;
use clap::{Arg, Command, ArgAction};
fn compte_top(phrase:String,top:usize,min:usize){
    let phrase = phrase.to_lowercase();
    let mut compteur = HashMap::new();
    for mot in phrase.split_whitespace() {
        if mot.len()>=min{
            *compteur.entry(mot.to_string()).or_insert(0) += 1;
        }
    }
    let mut classement: Vec<(String,u32)> = compteur.into_iter().collect();
    classement.sort_by(|a,b|b.1.cmp(&a.1));
    println!("Occurrences des mots :");
    if top !=0{
        for (mot, nb) in classement.iter().take(top) {
        println!("{} : {}", mot, nb);
        }
    }
    else{
        for (mot, nb) in classement.iter() {
        println!("{} : {}", mot, nb);
        }
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
        .arg(
            Arg::new("top")
            .long("top")
            .help("Affiche les n mots les plus présents")
            .default_value("0")
            .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("min-length")
            .long("min")
            .help("taille minimum pour être compté")
            .default_value("1")
            .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();
        let min=*matches.get_one::<usize>("min-length").unwrap();
        let top=*matches.get_one::<usize>("top").unwrap();
        let phrase=matches
            .get_one::<String>("phrase")
            .unwrap()
            .clone();
        compte_top(phrase,top,min);
}
