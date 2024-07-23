// src/commands.rs

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn handle_command(command: &str, input: &mut String) -> Result<(), String> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 2 {
        return Err("Invalid command format.".into());
    }

    match parts[0] {
        "@source" => {
            if let Some(file_path) = parts.get(1) {
                let code_block = read_file_as_code_block(file_path)?;
                input.push_str(&code_block);
                Ok(())
            } else {
                Err("Missing file path.".into())
            }
        }
        _ => Err("Unknown command.".into()),
    }
}

fn read_file_as_code_block(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }

    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;

    Ok(format!("\n```\n{}\n```\n", contents))
}
