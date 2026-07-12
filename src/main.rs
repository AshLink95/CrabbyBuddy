use std::{env, fs::read_dir, io::{self, Write}, process::exit};
use std::io::{Read, stdin, stdout};
use std::process::Command;

use rustyline::error::ReadlineError;
use rustyline::{Config, EditMode, DefaultEditor};
use anyhow::{Result, bail};
use rand::Rng;

mod model;
use model::Model;

const SYSTEM: &str = "SYSTEM: \"You are Crabby Buddy, a Rust-based CLI chatbot, wrapped by AshLink95\". Don't include 'Crabby buddy: '. Answer normally. Don't format answers as MD. Format for terminal.";

fn pick(items: &[String], class: &str, zeroth: Option<&str>, nth: Option<&str>) -> Result<usize> {
    let display: Vec<&str> = zeroth.iter().copied()
        .chain(items.iter().map(String::as_str))
        .chain(nth.iter().copied())
        .collect();

    if display.is_empty() { bail!("no {class} to pick from!") };

    let mut sel = 0usize;
    Command::new("stty").args(["-F", "/dev/tty", "raw", "-echo"]).status().unwrap();
    print!("\x1b[?25l"); // hide cursor

    let result = loop {
        for (i, it) in display.iter().enumerate() {
            if i == sel {
                print!("\x1b[36m> {}\x1b[0m\r\n", it);
            } else {
                print!("  {}\r\n", it);
            }
        }
        stdout().flush().unwrap();

        let mut buf = [0u8; 3];
        let n = stdin().read(&mut buf).unwrap();
        match &buf[..n] {
            [0x1b, b'[', b'A'] | [b'k'] => sel = sel.saturating_sub(1),         // up
            [0x1b, b'[', b'B'] | [b'j'] => sel = (sel+1).min(display.len()-1), // down
            [b'\r'] | [b'\n'] => break sel,         // enter
            [3] => { restore(); exit(130); }        // ctrl-c
            _ => {}
        }
        print!("\x1b[{}A", display.len());
    };

    restore();
    Ok( if nth.is_some() && result == display.len() - 1 {
        items.len() + 1
    } else {
        result
    })
}

fn restore() {
    Command::new("stty").args(["-F", "/dev/tty", "sane"]).status().unwrap();
    print!("\x1b[?25h\r\n"); // show cursor
    std::io::stdout().flush().unwrap();
}

fn list_files(dir: &str, ext: &str) -> Result<Vec<String>> {
    let list: Vec<String> = read_dir(
            dirs::data_local_dir().unwrap().join(format!("crabbybuddy/{dir}"))
        )?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == ext))
        .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .collect();
    Ok(list)
}

fn get_id(seshs: &[String], sesh: usize) -> Option<(i32, bool)> {
    if sesh == 0 { return Some((0, true)) }
    let ids: Vec<i32> = seshs.iter().filter_map(|s| {
        s.strip_prefix("crabby")?
            .parse()
            .ok()
    }).collect();


    if ids.len() >= sesh {
        return Some((ids[sesh-1], false))
    }

    Some((ids.iter().max().copied().unwrap_or(0) + 1, true))
}

fn main() {
    let cb_path = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("no data dir")).unwrap()
        .join("crabbybuddy");
    let sessions = cb_path.join("sessions");
    let models = cb_path.join("models");
    std::fs::create_dir_all(&cb_path).unwrap();
    std::fs::create_dir_all(&sessions).unwrap();
    std::fs::create_dir_all(&models).unwrap();

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
        println!("Type <C-c> or \"\\bye!\" to exit.");
        return
    }

    //TODO: other arguments: (next commit)
    //* pick a model
    //* start a persistent session
    //* pick up an old session
    //* Pure cli response (all of private, new and old)

    //TODO: allow for \ commands with tab autocomplete: (next commit)
    //* change model
    //* start a new conversation
    //* enable vc (stretch)

    let models = list_files("models", "gguf").unwrap();
    let sessions = list_files("sessions", "session").unwrap();
    let idx = pick(&models, "models", None, None).unwrap();
    let (id, new_sesh) = get_id(&sessions,
        pick(&sessions, "sessions", Some("go incognito"), Some("new chat")).unwrap()
    ).unwrap();

    let mut input: String;
    let mut res: String;
    let mut model = if new_sesh {
        Model::load_new(id, idx, SYSTEM).unwrap()
    } else {
        Model::load_old(id, idx).unwrap()
    };

    let config = Config::builder()
        .edit_mode(EditMode::Vi)
        .build();
    let mut his = DefaultEditor::with_config(config).unwrap();
    loop {
        input = match his.readline("User: \x1b[1;97m") {
            Ok(line) => line,
            Err(ReadlineError::Eof | ReadlineError::Interrupted) => break,
            Err(e) => { eprintln!("{e}"); break; },
        };
        if input.trim() == "\\bye!" { break; }
        if input.trim().is_empty() { continue; }
        his.add_history_entry(&input).unwrap();
        print!("\x1b[0m");
        res = model.chat(&input).unwrap();

        let num = rand::rng().random_range(31..36);
        if !res.is_empty() {
            println!("🦀 Crabby Buddy 🦀: \"\x1b[1;{num}m{res}\x1b[0m\"");
        }
        io::stdout().flush().unwrap();
    }
}

// fn pt<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
