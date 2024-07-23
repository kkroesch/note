# Note CLI Program

A CLI program for creating notes. The user enters lines, which are saved to a file after entering a single dot (".") on the input line. The file contains a YAML-like header with the title (first line of input), date (current date in ISO format), and author ($USER). The file is saved in the directory `$NOTES_DIR/drafts`.

## Features

- **Simple Note Taking:** Enter lines of text and save them with a single dot.
- **YAML-like Header:** Each file includes a header with the title, date, and author.
- **Custom Save Directory:** Notes are saved in the directory specified by the `$NOTES_DIR` environment variable.

## Usage

To run the program, ensure you have Rust installed, clone this repository, and use `cargo` to build and run the project.

### Build and Run

```sh
# Clone this repository
just install
```

### Create Notes

1. **Start the program:**
   ```sh
   note
   ```

2. **Enter your notes:**
   ```
   # Title of the note
   > This is the first line of the note.
   > This is the second line.
   > .
   ```

   After entering a single dot (".") on a line, the note will be saved.


## Installation with Justfile

You can use `just install` to build and install the program to `~/.local/bin`.


## Environment Variables

- `NOTES_DIR`: The directory where notes are saved. Defaults to the user's home directory if not set.

## Example

```sh
# Set the NOTES_DIR environment variable
export NOTES_DIR=~/my_notes
```

## Dependencies

- `dirs`: Access to directories (e.g., home directory).
- `chrono`: Working with date and time.
- `ansi_term`: Colored terminal output.
```
