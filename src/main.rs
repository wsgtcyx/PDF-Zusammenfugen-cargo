use clap::Parser;
use pdf_zusammenfugen_cargo::merge_pdfs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "pdf-zusammenfugen-cargo",
    version,
    about = "Fuegt mehrere PDF-Dateien zu einer Ausgabedatei zusammen."
)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "DATEI",
        help = "Pfad fuer die zusammengefuehrte Ausgabedatei."
    )]
    output: PathBuf,

    #[arg(
        value_name = "EINGABE_PDF",
        help = "Eine oder mehrere Eingabe-PDF-Dateien."
    )]
    inputs: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if cli.inputs.is_empty() {
        eprintln!("Fehler: Bitte mindestens eine Eingabe-PDF angeben.");
        std::process::exit(2);
    }

    match merge_pdfs(&cli.inputs, &cli.output) {
        Ok(()) => {
            println!(
                "Erfolg: {} Datei(en) nach {} zusammengefuehrt.",
                cli.inputs.len(),
                cli.output.display()
            );
        }
        Err(error) => {
            eprintln!("Fehler beim Zusammenfuegen: {error}");
            std::process::exit(1);
        }
    }
}
