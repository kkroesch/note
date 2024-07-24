# Justfile
# Einfache Justfile für Rust-Projekt, um Release-Builds zu erstellen und sie in ~/.local/bin zu kopieren.

# Definiere die Binärdatei und das Zielverzeichnis
BINARY := "note"
TARGET_DIR := "~/.local/bin"

release_notes:
    @git log --oneline $(git describe --tags --abbrev=0) | grep -E '(FEATURE|FIX)'

# Kompiliert das Projekt im Release-Modus
build:
    @cargo build --release

# Kopiert das kompilierte Binary in das Zielverzeichnis
install: build
    @cp target/release/{{BINARY}} {{TARGET_DIR}}

# Aufräumen
clean:
    @cargo clean
