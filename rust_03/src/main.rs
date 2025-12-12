use clap::{Parser, Subcommand};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const P: u64 = 0xD87FA3E291B4C7F3;
const G: u64 = 2;

fn mod_pow(base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result: u128 = 1;
    let mut base_u128 = base as u128;
    let modulus_u128 = modulus as u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base_u128) % modulus_u128;
        }
        base_u128 = (base_u128 * base_u128) % modulus_u128;
        exp /= 2;
    }
    result as u64
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }

    fn next_byte(&mut self) -> u8 {
        let a: u64 = 1103515245;
        let c: u64 = 12345;
        let m: u64 = 1u64 << 32;

        let next_val = (self.state.wrapping_mul(a).wrapping_add(c)) % m;
        self.state = next_val;
        ((next_val >> 16) & 0xFF) as u8
    }
}

#[derive(Parser)]
#[command(
    author="Olivier",
    version="1.0",
    about="Stream cipher chat with Diffie-Hellman key generation",
    long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server { port: u16 },
    Client { addr: String },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server { port } => {
            let address = format!("0.0.0.0:{}", port);
            println!("[SERVER] Listening on {}", address);
            let listener = TcpListener::bind(address)?;
            println!("[SERVER] Waiting for client...");

            if let Ok((stream, addr)) = listener.accept() {
                println!("[CLIENT] Connected from {}", addr);
                handle_connection(stream)?;
            }
        }
        Commands::Client { addr } => {
            println!("[CLIENT] Connecting to {}...", addr);
            let stream = TcpStream::connect(addr)?;
            println!("[CLIENT] Connected!");
            handle_connection(stream)?;
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!("\n[DH] Starting key exchange...");
    println!("[DH] Using hardcoded DH parameters:");
    println!("p = {:X} (64-bit prime - public)", P);
    println!("g = {} (generator - public)", G);

    let private_key: u64 = rand::random();
    println!("\n[DH] Generating our keypair...");
    println!("private_key = {} (random u64)", private_key);

    let public_key = mod_pow(G, private_key, P);
    println!("public_key = g^private_key mod p");
    println!("           = {:X}", public_key);

    println!("\n[DH] Exchanging keys...");

    println!("[NETWORK] Sending public key (8 bytes)...");
    stream.write_all(&public_key.to_be_bytes())?;

    let mut buffer = [0u8; 8];
    stream.read_exact(&mut buffer)?;
    let their_public_key = u64::from_be_bytes(buffer);
    println!("[NETWORK] Received public key (8 bytes) ✓");
    println!(" - Receive their public: {:X}", their_public_key);

    println!("\n[DH] Computing shared secret...");
    println!("Formula: secret = (their_public)^(our_private) mod p");

    let shared_secret = mod_pow(their_public_key, private_key, P);
    println!("\nSecret = {:X}", shared_secret);
    println!("[VERIFY] Both sides computed the same secret ✓");

    println!("\n[STREAM] Generating keystream from secret...");
    println!("Algorithm: LCG (a=1103515245, c=12345, m=2^32)");
    println!("Seed: secret = {:X}", shared_secret);

    let cipher = Arc::new(Mutex::new(Lcg::new(shared_secret)));

    {
        let mut temp_cipher = Lcg::new(shared_secret);
        print!("Keystream: ");
        for _ in 0..10 {
            print!("{:02X} ", temp_cipher.next_byte());
        }
        println!("...");
    }

    println!("\n✓ Secure channel established!\n");

    let stream_read = stream.try_clone()?;
    let cipher_read = Arc::clone(&cipher);

    thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        let mut socket = stream_read;
        loop {
            match socket.read(&mut buffer) {
                Ok(0) => {
                    println!("\n[DISCONNECT] Peer disconnected.");
                    std::process::exit(0);
                }
                Ok(n) => {
                    let encrypted_msg = &buffer[0..n];
                    println!("\n[NETWORK] Received encrypted message ({} bytes)", n);

                    let mut decrypted_msg = Vec::new();
                    let mut keystream_bytes = Vec::new();

                    let mut cipher_lock = cipher_read.lock().unwrap();

                    for &byte in encrypted_msg {
                        let k = cipher_lock.next_byte();
                        keystream_bytes.push(k);
                        decrypted_msg.push(byte ^ k);
                    }
                    drop(cipher_lock);

                    println!("\n[DECRYPT]");
                    print!("Cipher: ");
                    for b in encrypted_msg {
                        print!("{:02x} ", b);
                    }
                    print!("\nKey:    ");
                    for b in &keystream_bytes {
                        print!("{:02x} ", b);
                    }
                    print!(" (keystream position updated)\nPlain:  ");
                    for b in &decrypted_msg {
                        print!("{:02x} ", b);
                    }
                    println!();

                    if let Ok(msg_str) = String::from_utf8(decrypted_msg) {
                        println!("\n[PEER] > {}", msg_str.trim());
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    });

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let msg = input.trim();
        if msg.is_empty() {
            continue;
        }

        let plain_bytes = msg.as_bytes();
        let mut encrypted_bytes = Vec::new();
        let mut keystream_bytes = Vec::new();

        let mut cipher_lock = cipher.lock().unwrap();

        for &byte in plain_bytes {
            let k = cipher_lock.next_byte();
            keystream_bytes.push(k);
            encrypted_bytes.push(byte ^ k);
        }
        drop(cipher_lock);

        println!("\n[ENCRYPT]");
        print!("Plain:  ");
        for b in plain_bytes {
            print!("{:02x} ", b);
        }
        print!("\nKey:    ");
        for b in &keystream_bytes {
            print!("{:02x} ", b);
        }
        print!(" (keystream position updated)\nCipher: ");
        for b in &encrypted_bytes {
            print!("{:02x} ", b);
        }
        println!();

        println!(
            "[NETWORK] Sending encrypted message ({} bytes)...",
            encrypted_bytes.len()
        );
        stream.write_all(&encrypted_bytes)?;
        println!("[-] Sent {} bytes", encrypted_bytes.len());
    }
}
