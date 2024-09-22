# CLI Example - Programma di Saluto Personalizzato

Questo progetto è un'applicazione da riga di comando (CLI) scritta in Rust che permette di salutare più persone con saluti personalizzati. Utilizza la libreria Clap per la gestione degli argomenti da riga di comando.

## Funzionalità

- Saluta multiple persone con saluti personalizzati
- Permette di specificare il numero di ripetizioni per i saluti
- Supporta l'output su file o su console
- Include una modalità verbose per informazioni dettagliate
- Validazione degli input per nomi e saluti
- Esempio di utilizzo integrato
- Suite di test completa per verificare tutte le funzionalità

## Requisiti

- Rust (edizione 2021 o superiore)
- Cargo (gestore di pacchetti di Rust)

## Installazione

1. Clona questo repository:
   ```
   git clone https://github.com/tuousername/cli-example.git
   cd cli-example
   ```

2. Compila il progetto:
   ```
   cargo build --release
   ```

## Utilizzo

Esegui il programma con:

```
cargo run -- [OPZIONI]
```

Opzioni disponibili:
- `-n, --names <NAMES>`: Lista di nomi separati da virgola (es. "Mario,Anna")
- `-g, --greetings <GREETINGS>`: Lista di saluti separati da virgola (es. "Salve,Ciao")
- `-r, --repeat <REPEAT>`: Numero di volte in cui ripetere il saluto (predefinito: 1)
- `-o, --output <OUTPUT>`: Percorso del file di output per salvare l'output (opzionale)
- `-v, --verbose`: Abilita l'output dettagliato
- `-e, --example`: Mostra un esempio di utilizzo del programma e termina l'esecuzione
- `-h, --help`: Mostra l'aiuto
- `-V, --version`: Mostra la versione del programma
