use std::{io::{self, Write}, env};
use anyhow::Result;
use rand::Rng;

mod client;
use client::ApiFreeLLM;

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
    loop {
        if !timeout {
            print!("User: \x1b[1;97m");
            io::stdout().flush().unwrap();
            input = String::new();
            io::stdin().read_line(&mut input).expect("User input Ig!");
            print!("\x1b[0m");
        }
        let api_bod = ApiFreeLLM::new(input.trim()).await;
        res = api_bod.get_resp().to_string();
        if res.is_empty() {
            println!("Big dawg responded with status '\x1b[1;3;31m{}\x1b[0m', wait a sec...", api_bod.get_stat());
            io::stdout().flush().unwrap();
            // break;
            tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            timeout = true;
        } else {
            timeout = false;
        }
        if input.trim() == "Bye!" {
            break;
        }
        let num = rand::rng().random_range(31..36);
        println!("Big dawg: \"\x1b[1;{num}m{res}\x1b[0m\"");
        io::stdout().flush().unwrap();
    }
    Ok(())
}

// fn pt<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
