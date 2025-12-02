use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Olivier", about = "Dit bonjour avec des options")]
struct Args {

    #[arg(required = false, index = 1)]
    name: Option<String>,

    #[arg(long, short)]
    upper: bool,

    #[arg(long, short, default_value_t = 1)]
    repeat: usize,
}

fn main() {
    let args = Args::parse();
    
    // Le reste du code est correct
    for _ in 0..args.repeat {
        hello(args.name.as_deref(), args.upper);
    }
}

fn hello(name: Option<&str>, upper: bool) {
    let mut message = match name {
        Some(name) => format!("Hello, {}!", name),
        None => "Hello, World!".to_string(),
    };
    if upper {
        message = message.to_uppercase();
    }
    println!("{}", message);
}