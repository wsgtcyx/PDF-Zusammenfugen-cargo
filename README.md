# PDF-Zusammenfugen Cargo

Ein Rust-Tool zum **pdf zusammenfuegen**, **pdf verbinden** und **pdf mergen** fuer lokale Workflows.

Wenn du lieber direkt im Browser arbeitest, findest du hier die Online-Variante:
[PDF zusammenfuegen online](https://pdfzus.de/)

## Funktionen

- Fuegt mehrere PDF-Dateien zu einer Datei zusammen
- Nutzbar als CLI und als Rust-Library
- Strikte Eingabevalidierung mit klaren Fehlermeldungen
- Unit-Tests fuer die Kernlogik

## Installation

### CLI lokal installieren

```bash
cargo install --path .
```

### Als Library nutzen

```bash
cargo add pdf-zusammenfugen-cargo
```

## CLI-Nutzung

```bash
pdf-zusammenfugen-cargo -o merged.pdf teil1.pdf teil2.pdf teil3.pdf
```

## Library-Beispiel

```rust
use pdf_zusammenfugen_cargo::merge_pdfs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_files = vec![
        PathBuf::from("teil1.pdf"),
        PathBuf::from("teil2.pdf"),
    ];

    merge_pdfs(&input_files, "merged.pdf")?;
    Ok(())
}
```

## Entwicklung

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

## Lizenz

MIT
