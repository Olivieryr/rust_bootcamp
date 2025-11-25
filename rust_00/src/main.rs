use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();    
    hello(args.get(1));
}

fn hello(name: Option<&String>){
    match name{
        Some(name)=>println!("Bonjour, {} !",name),
        None=>println!("Bonjour le monde !"),
    }
}
