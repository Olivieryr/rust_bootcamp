use clap::{Arg, Command, ArgAction};

fn main() {
    let matches = Command::new("hello_app")
        .version("1.0")
        .author("Ton Nom")
        .about("Dit bonjour avec des options")
        .arg(
            Arg::new("name")
                .help("Nom de la personne à saluer")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("upper")
                .long("upper")
                .help("Met le message en majuscules")
                .action(ArgAction::SetTrue), // <-- ici, pour un flag booléen
        )
        .arg(
            Arg::new("repeat")
                .long("repeat")
                .help("Nombre de répétitions")
                .value_parser(clap::value_parser!(usize)), // prend une valeur
        )
        .get_matches();

    let name = matches.get_one::<String>("name").map(|s| s.as_str());
    let upper = matches.get_flag("upper"); // <-- récupère le booléen
    let repeat = *matches.get_one::<usize>("repeat").unwrap_or(&1);

    for _ in 0..repeat {
        hello(name, upper);
    }
}

fn hello(name: Option<&str>, upper: bool) {
    let mut message = match name {
        Some(name) => format!("Bonjour, {} !", name),
        None => "Bonjour le monde !".to_string(),
    };
    if upper {
        message = message.to_uppercase();
    }
    println!("{}", message);
}
