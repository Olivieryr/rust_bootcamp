use clap::{CommandFactory, Parser};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

#[derive(Parser, Debug)]
#[command(
    name = "Hextool",
    author = "Olivier",
    version = "1.0",
    about = "Outil de manipulation hexadecimal",
    help_template = "\
    {before-help}{name} (v{version})
    Author: {author}

    About:
    {about}

    Usage:
    {usage}

    Args:
    {all-args}

    {after-help}"
)]

struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long, group = "mode")]
    read: bool,

    #[arg(short, long, group = "mode")]
    write: Option<String>,

    #[arg(short, long, default_value_t = 0, value_parser = parse_offset)]
    offset: u64,

    #[arg(short, long)]
    size: Option<usize>,
}

fn parse_offset(src: &str) -> Result<u64, String> {
    if src.starts_with("0x") {
        u64::from_str_radix(&src[2..], 16)
            .map_err(|_| format!("Offset hexadécimal invalide: {}", src))
    } else {
        src.parse::<u64>()
            .map_err(|_| format!("Offset décimal invalide: {}", src))
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.read {
        let size = args.size.unwrap_or(256);
        match read_file(&args.file, args.offset, size) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Erreur de lecture: {}", e);
                Err(e)
            }
        }
    } else if let Some(hex_string) = args.write {
        match write_file(&args.file, args.offset, &hex_string) {
            Ok(_) => {
                println!("\n✅ Successfully written");
                Ok(())
            }
            Err(e) => {
                eprintln!("Erreur d'écriture: {}", e);
                Err(e)
            }
        }
    } else {
        println!("Mode non spécifié. Utilisez -r (read) ou -w (write).");
        Args::command().print_help()?;
        Ok(())
    }
}

fn read_file(filepath: &str, offset: u64, size: usize) -> io::Result<()> {
    let mut file = File::open(filepath).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Fichier non trouvé: {}", e),
        )
    })?;

    file.seek(SeekFrom::Start(offset))?;

    let mut buffer = vec![0u8; size];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);

    if bytes_read == 0 {
        println!("Aucun octet lu à l'offset {:#08X}", offset);
        return Ok(());
    }

    print_hex_dump(&buffer, offset);
    Ok(())
}

fn print_hex_dump(data: &[u8], start_offset: u64) {
    const BYTES_PER_LINE: usize = 16;

    for (i, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
        let current_offset = start_offset + (i * BYTES_PER_LINE) as u64;

        print!("{:#08X}: ", current_offset);

        let hex_part: String = chunk.iter().map(|byte| format!("{:02X} ", byte)).collect();

        print!("{: <48}", hex_part);

        let ascii_part: String = chunk
            .iter()
            .map(|&byte| {
                if byte >= 0x20 && byte <= 0x7E {
                    byte as char
                } else {
                    '.'
                }
            })
            .collect();

        println!("|{}|", ascii_part);
    }
}
fn write_file(filepath: &str, offset: u64, hex_string: &str) -> io::Result<()> {
    let bytes_to_write = match hex_to_bytes(hex_string) {
        Ok(b) => b,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
    };

    let write_size = bytes_to_write.len();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filepath)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Erreur d'ouverture: {}", e)))?;

    file.seek(SeekFrom::Start(offset))?;
    file.write_all(&bytes_to_write)?;

    println!("Writing {} bytes at offset {:#08X}", write_size, offset);
    println!("Hex: {}", hex_string);

    let ascii_preview: String = bytes_to_write
        .iter()
        .map(|&byte| {
            if byte >= 0x20 && byte <= 0x7E {
                byte as char
            } else {
                '.'
            }
        })
        .collect();
    println!("ASCII: {}", ascii_preview);

    println!("✓ Successfully written");

    Ok(())
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let cleaned_hex = hex.replace(" ", "");
    if cleaned_hex.len() % 2 != 0 {
        return Err("Chaîne hexadécimale invalide (longueur impaire)".to_string());
    }

    (0..cleaned_hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&cleaned_hex[i..i + 2], 16)
                .map_err(|_| format!("Hex invalide: {}", &cleaned_hex[i..i + 2]))
        })
        .collect()
}
