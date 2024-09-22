use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, Write};

const MAX_LENGTH: usize = 50;  // Limite massimo per i nomi e i saluti
const MIN_LENGTH: usize = 2;   // Lunghezza minima per i nomi e i saluti

/// Programma CLI di esempio che utilizza Clap per salutare più persone con saluti personalizzati.
#[derive(Parser)]
#[command(name = "CLI Example", version = "1.0", author = "Il Tuo Nome", about = "Un programma per salutare più persone con saluti personalizzati.")]
struct Cli {
    /// Lista di nomi separati da virgola (es. "Mario,Anna")
    #[arg(short, long, required_unless_present = "example")]
    names: Option<String>,

    /// Lista di saluti separati da virgola (es. "Salve,Ciao")
    #[arg(short, long, required_unless_present = "example")]
    greetings: Option<String>,

    /// Numero di volte in cui ripetere il saluto (predefinito: 1)
    #[arg(short, long, default_value_t = 1)]
    repeat: i32,

    /// Percorso del file di output per salvare l'output (opzionale)
    #[arg(short, long)]
    output: Option<String>,

    /// Abilita l'output dettagliato (verbose)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,

    /// Mostra un esempio di utilizzo del programma e termina l'esecuzione
    #[arg(short = 'e', long = "example", action = clap::ArgAction::SetTrue)]
    example: bool,
}

fn validate_input(input: &str, input_type: &str) {
    let alphanumeric_regex = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap();
    for part in input.split(',').map(|p| p.trim()) {
        // Controllo dei caratteri validi
        if !alphanumeric_regex.is_match(part) {
            eprintln!("Errore: il {} '{}' contiene caratteri non validi. Sono ammessi solo caratteri alfanumerici e spazi.", input_type, part);
            std::process::exit(1);
        }
        // Controllo lunghezza minima
        if part.len() < MIN_LENGTH {
            eprintln!("Errore: il {} '{}' è troppo corto. La lunghezza minima consentita è {} caratteri.", input_type, part, MIN_LENGTH);
            std::process::exit(1);
        }
        // Controllo lunghezza massima
        if part.len() > MAX_LENGTH {
            eprintln!("Errore: il {} '{}' è troppo lungo. La lunghezza massima consentita è {} caratteri.", input_type, part, MAX_LENGTH);
            std::process::exit(1);
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    // Se l'utente richiede un esempio, stamparlo e terminare l'esecuzione
    if args.example {
        println!("Esempio di utilizzo:");
        println!("  cargo run -- --names \"Mario,Anna\" --greetings \"Salve,Ciao\" --repeat 3");
        println!("  cargo run -- --names \"Mario,Anna\" --greetings \"Salve,Ciao\" --repeat 3 --output saluti.txt");
        return Ok(());
    }

    // Controllo che il valore di repeat non sia negativo
    if args.repeat <= 0 {
        eprintln!("Errore: il valore di --repeat deve essere un numero positivo.");
        std::process::exit(1);
    }

    // Verifica che i nomi e i saluti siano stati forniti correttamente
    let names = args.names.expect("I nomi sono obbligatori.");
    let greetings = args.greetings.expect("I saluti sono obbligatori.");

    // Validazione dei nomi e dei saluti
    validate_input(&names, "nome");
    validate_input(&greetings, "saluto");

    // Tentativo di aprire o creare il file di output, con gestione degli errori
    let mut output: Box<dyn Write> = if let Some(output_file) = args.output {
        match File::create(&output_file) {
            Ok(file) => Box::new(file),
            Err(e) => {
                eprintln!("Errore: impossibile creare il file '{}': {}", output_file, e);
                std::process::exit(1);
            }
        }
    } else {
        Box::new(io::stdout()) // Stampa su stdout
    };

    // Dividi la stringa dei nomi e dei saluti basandoti sulla virgola
    let names: Vec<&str> = names.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let greetings: Vec<&str> = greetings.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    // Gestione errore se ci sono nomi o saluti vuoti
    if names.is_empty() {
        eprintln!("Errore: la lista dei nomi è vuota o contiene solo spazi.");
        std::process::exit(1);
    }

    if greetings.is_empty() {
        eprintln!("Errore: la lista dei saluti è vuota o contiene solo spazi.");
        std::process::exit(1);
    }

    // Gestione errore se il numero di nomi e saluti non corrisponde
    if names.len() != greetings.len() {
        eprintln!("Attenzione: il numero di nomi ({}) e di saluti ({}) non corrisponde. Il programma utilizzerà il ciclo per abbinarli.", names.len(), greetings.len());
    }

    if args.verbose {
        writeln!(output, "Modalità dettagliata attivata.")?;
        writeln!(output, "Ripetizioni del saluto: {}", args.repeat)?;
        writeln!(output, "Saluti: {:?}", greetings)?;
        writeln!(output, "Nomi: {:?}", names)?;
    }

    // Cicla sulle ripetizioni e alterna tra saluti e nomi ciclicamente
    for i in 0..args.repeat {
        let greeting = greetings[i as usize % greetings.len()];
        let name = names[i as usize % names.len()];
        writeln!(output, "{} {}!", greeting, name)?;
    }

    Ok(())
}
