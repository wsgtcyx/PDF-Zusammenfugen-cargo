# PDF-Zusammenfugen Cargo

Ein praxistaugliches Rust-Tool zum Zusammenfuegen mehrerer PDF-Dateien ueber CLI und Bibliothek.

Mehr zur Browser-Loesung fuer Endnutzer: [pdfzus.de](https://pdfzus.de/)

## Warum dieses Tool?

Dieses Crate liefert eine klare, lokale Merge-Logik fuer PDF-Workflows in Rust-Projekten.
Wenn du eine direkte Browser-Variante ohne Upload suchst, siehe [pdfzus.de](https://pdfzus.de/).

## Features

- Mehrere PDF-Dateien in eine Ausgabe zusammenfuehren
- Nutzbar als Library (`merge_pdfs`) und als CLI
- Validierung von Eingabepfaden
- Unit-Tests mit echten PDF-Testdateien

## Installation

### Als CLI aus dem Quellcode

```bash
cargo install --path .
```

### Als Abhaengigkeit

```bash
cargo add pdf-zusammenfugen-cargo
```

## CLI Nutzung

```bash
pdf-zusammenfugen-cargo -o merged.pdf teil1.pdf teil2.pdf teil3.pdf
```

## Library Nutzung

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
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo package
```

## Lizenz

MIT
