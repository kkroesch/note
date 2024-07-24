// note.rs
// A CLI program for creating notes. The user enters lines,
// which are saved to a file after entering a single dot (".") on the input line. The file
// contains a YAML-like header with the title (first line of input),
// date (current date in ISO format), and author ($USER).
// The file is saved in the directory $NOTES_DIR/drafts.
//

// Used libraries:
// - dirs: Access to directories (e.g., home directory).
// - chrono: Working with date and time.
// - ansi_term: Colored terminal output.
// - rustyline: Readline-like line editor.

mod commands;

use ansi_term::Colour::{Fixed, Green, Red};
use chrono::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;

    // Read the NOTES_DIR environment variable or use the home directory as a fallback.
    let notes_dir = env::var("NOTES_DIR").unwrap_or_else(|_| {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .to_str()
            .unwrap()
            .to_string()
    });

    let drafts_dir = PathBuf::from(&notes_dir).join("drafts");

    fs::create_dir_all(&drafts_dir)?;
    println!(
        "{}",
        Fixed(8).paint("✏️ Enter your notes. Type a single '.' on a line to save. CTRL-C to abort.")
    );

    let mut input = String::new();
    // Read the first line with the prompt '#'
    let first_line = rl.readline("TITLE: ")?;
    let title = first_line.trim();

    // Add the first line to the overall input text
    input.push_str(&format!("# {}\n", first_line));

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                if line.trim() == "." {
                    break;
                }

                // Check if the line is a command
                if line.trim().starts_with('@') {
                    if let Err(err) = commands::handle_command(line.trim(), &mut input) {
                        eprintln!("{}", Red.paint(err));
                    }
                } else {
                    input.push_str(&line);
                    input.push('\n'); // Add a newline character after each line
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    let filename = title.trim().to_owned() + ".md";
    let file_path = drafts_dir.join(&filename);
    let mut file = File::create(&file_path)?;

    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let date = Utc::now().to_rfc3339();

    let header = format!(
        "---\ntitle: {}\ndate: {}\nauthor: {}\n---\n\n",
        title, date, user
    );

    file.write_all(header.as_bytes())?;
    file.write_all(input.as_bytes())?;
    println!("{}", Green.paint(format!("Saved to {:?}", file_path)));
    Ok(())
}
