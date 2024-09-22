use assert_cmd::Command;
use std::fs;
use predicates::prelude::*;

#[test]
fn test_example_option() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    cmd.arg("--example")
        .assert()
        .success()
        .stdout(predicates::str::contains("Esempio di utilizzo:"))
        .stdout(predicates::str::contains("cargo run -- --names \"Mario,Anna\" --greetings \"Salve,Ciao\" --repeat 3"))
        .stdout(predicates::str::contains("cargo run -- --names \"Mario,Anna\" --greetings \"Salve,Ciao\" --repeat 3 --output saluti.txt"));
}

#[test]
fn test_basic_greetings() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    cmd.args(&["--names", "Mario,Anna", "--greetings", "Salve,Ciao", "--repeat", "2"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Salve Mario!"))
        .stdout(predicates::str::contains("Ciao Anna!"));
}

#[test]
fn test_repeat_option() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    let output = cmd.args(&["--names", "Mario", "--greetings", "Salve", "--repeat", "3"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let output_str = String::from_utf8_lossy(&output);
    let count = output_str.matches("Salve Mario!").count();
    assert_eq!(count, 3);
}

#[test]
fn test_output_to_file() {
    let output_file = "test_output.txt";
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    cmd.args(&["--names", "Mario", "--greetings", "Salve", "--output", output_file])
        .assert()
        .success();

    let output_content = fs::read_to_string(output_file).expect("Unable to read output file");
    assert!(output_content.contains("Salve Mario!"));

    // Clean up
    fs::remove_file(output_file).expect("Unable to delete test output file");
}

#[test]
fn test_verbose_option() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    cmd.args(&["--names", "Mario", "--greetings", "Salve", "--verbose"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Modalità dettagliata attivata"))
        .stdout(predicates::str::contains("Ripetizioni del saluto: 1"))
        .stdout(predicates::str::contains("Saluti: [\"Salve\"]"))
        .stdout(predicates::str::contains("Nomi: [\"Mario\"]"))
        .stdout(predicates::str::contains("Salve Mario!"));
}

#[test]
fn test_empty_names() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    cmd.args(&["--names", "", "--greetings", "Salve"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il nome '' contiene caratteri non validi. Sono ammessi solo caratteri alfanumerici e spazi."));
}

#[test]
fn test_empty_greetings() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    cmd.args(&["--names", "Mario", "--greetings", ""])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il saluto '' contiene caratteri non validi. Sono ammessi solo caratteri alfanumerici e spazi."));
}

#[test]
fn test_unequal_names_and_greetings() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    cmd.args(&["--names", "Mario,Anna", "--greetings", "Salve"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Salve Mario!"))
        .stderr(predicates::str::contains("Attenzione: il numero di nomi (2) e di saluti (1) non corrisponde"));
}

#[test]
fn test_long_name() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    let long_name = "Mario".repeat(11); // Genera un nome troppo lungo
    cmd.args(&["--names", &long_name, "--greetings", "Salve"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il nome 'MarioMarioMarioMarioMarioMarioMarioMarioMarioMarioMario' è troppo lungo"));
}

#[test]
fn test_long_greeting() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    let long_greeting = "Salve".repeat(11); // Genera un saluto troppo lungo
    cmd.args(&["--names", "Mario", "--greetings", &long_greeting])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il saluto 'SalveSalveSalveSalveSalveSalveSalveSalveSalveSalveSalve' è troppo lungo"));
}

#[test]
fn test_non_numeric_repeat() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    cmd.args(&["--names", "Mario", "--greetings", "Salve", "--repeat", "abc"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("invalid value 'abc' for '--repeat <REPEAT>': invalid digit found in string"));
}

#[test]
fn test_negative_repeat() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();
    cmd.args(&["--names", "Mario", "--greetings", "Salve", "--repeat", "-3"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("unexpected argument '-3'"));
}

#[test]
fn test_empty_names_input() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    cmd.args(&["--names", "", "--greetings", "Salve", "--repeat", "1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il nome '' contiene caratteri non validi. Sono ammessi solo caratteri alfanumerici e spazi."));
}

#[test]
fn test_empty_greetings_input() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    cmd.args(&["--names", "Mario", "--greetings", "", "--repeat", "1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il saluto '' contiene caratteri non validi. Sono ammessi solo caratteri alfanumerici e spazi."));
}

#[test]
fn test_unwritable_file() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    // Proviamo a scrivere in una directory non esistente o non scrivibile
    let output_file = if cfg!(target_os = "windows") {
        "C:\\InvalidPath\\test_output.txt" // Un percorso sicuramente non scrivibile su Windows
    } else {
        "/invalid_path/test_output.txt" // Un percorso non scrivibile su Linux/Mac
    };

    cmd.args(&["--names", "Mario", "--greetings", "Salve", "--output", output_file])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: impossibile creare il file"));
}


#[test]
fn test_directory_instead_of_file() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    // Forniamo una directory invece di un file
    let output_file = if cfg!(target_os = "windows") {
        "C:\\Windows" // Una directory invece di un file su Windows
    } else {
        "/tmp" // Una directory invece di un file su Linux/Mac
    };

    cmd.args(&["--names", "Mario", "--greetings", "Salve", "--output", output_file])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: impossibile creare il file"));
}

#[test]
fn test_invalid_file_path() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    // Un percorso di file contenente caratteri non validi
    let output_file = if cfg!(target_os = "windows") {
        "C:\\Invalid<>Path\\output.txt"  // Percorso non valido su Windows
    } else {
        "/invalid|path/output.txt"  // Percorso non valido su Linux/Mac
    };

    cmd.args(&["--names", "Mario", "--greetings", "Salve", "--output", output_file])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: impossibile creare il file"));
}

#[test]
fn test_invalid_repeat_argument() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    // Proviamo a passare un valore non numerico o malformato a --repeat
    cmd.args(&["--names", "Mario", "--greetings", "Salve", "--repeat", "abc!@#"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("invalid digit found in string"));
}

#[test]
fn test_invalid_name_with_special_characters() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    cmd.args(&["--names", "@@@", "--greetings", "Salve", "--repeat", "1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il nome '@@@' contiene caratteri non validi. Sono ammessi solo caratteri alfanumerici e spazi."));
}

#[test]
fn test_invalid_greeting_with_special_characters() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    cmd.args(&["--names", "Mario", "--greetings", "***", "--repeat", "1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il saluto '***' contiene caratteri non validi. Sono ammessi solo caratteri alfanumerici e spazi."));
}

#[test]
fn test_name_too_short() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    cmd.args(&["--names", "A", "--greetings", "Salve", "--repeat", "1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il nome 'A' è troppo corto. La lunghezza minima consentita è 2 caratteri."));
}

#[test]
fn test_greeting_too_short() {
    let mut cmd = Command::cargo_bin("cli_example").unwrap();

    cmd.args(&["--names", "Mario", "--greetings", "B", "--repeat", "1"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Errore: il saluto 'B' è troppo corto. La lunghezza minima consentita è 2 caratteri."));
}
