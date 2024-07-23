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

mod commands;

use ansi_term::Colour::{Fixed, Green, Red};
use chrono::prelude::*;
use dirs::home_dir;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

fn main() {
    // Reads the NOTES_DIR environment variable or uses the home directory as a fallback.

    let notes_dir = env::var("NOTES_DIR").unwrap_or_else(|_| {
        home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .to_str()
            .unwrap()
            .to_string()
    });

    let drafts_dir = PathBuf::from(&notes_dir).join("drafts");

    if let Err(e) = std::fs::create_dir_all(&drafts_dir) {
        eprintln!(
            "{}",
            Red.paint(format!("Failed to create drafts directory: {}", e))
        );
        return;
    }

    println!(
        "{}",
        Fixed(8)
            .paint("✏️ Enter your note. First line is title. Type a single '.' on a line to save.")
    );
    let mut input = String::new();
    let stdin = io::stdin();

    // Read first line with prompt '#'
    print!("# ");
    io::stdout().flush().unwrap();
    let mut first_line = String::new();
    stdin
        .lock()
        .read_line(&mut first_line)
        .expect("Failed to read line");

    // remove line breaks
    let first_line = first_line.trim();

    // Add Markdown title
    input.push_str(&format!("# {}\n", first_line));

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin
            .lock()
            .read_line(&mut line)
            .expect("Failed to read line");

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
        }
    }

    if !first_line.is_empty() {
        let filename = first_line.replace(" ", "_") + ".md";
        let file_path = drafts_dir.join(&filename);
        let mut file = File::create(&file_path).expect("Failed to create file");

        let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        let date = Utc::now().to_rfc3339();

        let header = format!(
            "---\ntitle: {}\ndate: {}\nauthor: {}\n---\n\n",
            first_line, date, user
        );

        if let Err(e) = file.write_all(header.as_bytes()) {
            eprintln!(
                "{}",
                Red.paint(format!("Failed to write header to file: {}", e))
            );
            return;
        }
        if let Err(e) = file.write_all(input.as_bytes()) {
            eprintln!("{}", Red.paint(format!("Failed to write to file: {}", e)));
            return;
        }
        println!("{}", Green.paint(format!("Saved to {:?}", file_path)));
    } else {
        println!("{}", Red.paint("No input received."));
    }
}
