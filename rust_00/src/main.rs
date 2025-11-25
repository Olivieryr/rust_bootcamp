use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name=args.iter().skip(1).find(|arg| !arg.starts_with("--")).map(|arg|arg.as_str());  
    let upper = args.iter().any(|arg| arg=="--upper");  
    hello(name,upper);
}
fn hello(name: Option<&str>,upper:bool){
    let mut message = match name{
        Some(name)=>format!("Bonjour, {} !",name),
        None=>"Bonjour le monde !".to_string(),
    };
    if upper{
        message=message.to_uppercase();
    }
    println!("{}",message);
}
