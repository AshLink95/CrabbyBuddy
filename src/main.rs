use std::{io::{self, Write}, env};
use anyhow::Result;
use rand::Rng;

mod client;
use client::ApiFreeLLM;

/// How many times the request is resent in the case of an eror
const COUNT: u8 = 2;
/// System prompt
const SYSTEM: &str = "SYSTEM: \"You are Crabby Buddy, a Rust-based CLI chatbot, wrapped by AshLink95\". Don't include 'Crabby buddy: '. Answer normally.";

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("AI Chat CLI Tool");
        println!();
        println!("Usage: {} [OPTIONS]", args[0]);
        println!();
        println!("Options:");
        println!("  -h, --help     Show this help message");
        println!();
        println!("Type your messages and press Enter to chat.");
        println!("Type 'Bye!' to exit.");
        return Ok(());
    }

    let mut input = String::new();
    let mut res: String;
    let mut timeout = false;
    let mut message = String::from(SYSTEM);
    let mut count = COUNT;
    loop {
        if !timeout {
            print!("User: \x1b[1;97m");
            io::stdout().flush().unwrap();
            input = String::new();
            io::stdin().read_line(&mut input).expect("User input Ig!");
            print!("\x1b[0m");
            message += "User: ";
            message += input.trim();
            message += "\", ";
        }
        // change the argument to input.trim() for context-free chat bot
        let api_bod = ApiFreeLLM::new(message.trim()).await;
        res = api_bod.get_resp().to_string();
        if res.is_empty() && count > 0 {
            println!("Big dawg responded with status '\x1b[1;3;31m{}\x1b[0m', wait a sec...", api_bod.get_stat());
            io::stdout().flush().unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            timeout = true;
            count -= 1;
        } else {
            timeout = false;
            count = COUNT;
        }
        if input.trim() == "Bye!" {
            break;
        }
        let num = rand::rng().random_range(31..36);
        if !res.is_empty() {
            println!("🦀 Crabby Buddy 🦀: \"\x1b[1;{num}m{res}\x1b[0m\"");
            message += "Crabby Buddy: \"";
            message += res.trim();
            message += "\", ";
        }
        io::stdout().flush().unwrap();
    }
    Ok(())
}

// fn pt<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
