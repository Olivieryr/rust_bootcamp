use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();  
    let upper = args.iter().any(|arg| arg=="--upper");  
    let pos=args.iter().position(|arg| arg=="--repeat");
    let repeat = match pos{
        Some(pos)=>{
            if pos+1 <args.len(){
                match args[pos+1].parse::<usize>(){
                    Ok(n)=>n,
                    Err(_)=>{
                        eprintln!("Erreur: --repeat doit être suivi d'un nombre");
                        return;
                    }
                }
            }
            else{
                eprintln!("Erreur: --repeat doit être suivi d'un nombre");
                return;
            }
        }
        None=>1,
    };
    let name=args.iter().enumerate().skip(1).find(|(i,arg)| !arg.starts_with("--")&&Some(*i)!=pos.map(|p| p+1)).map(|(_i,arg)|arg.as_str());
    for _ in 0..repeat{
            hello(name,upper);
    }
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
