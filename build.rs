use std::fs;
use std::path::Path;

fn main() {
    // Hier gehe ich davon aus, dass sich dein assets-Ordner im Projektstammverzeichnis befindet.
    let src_dir = Path::new("assets"); // Relativer Pfad zum assets-Ordner im Projektstammverzeichnis
    let dest_dir = Path::new("target/release/assets");

    // Überprüfen, ob das Zielverzeichnis existiert, und falls nicht, erstellen
    if !dest_dir.exists() {
        if let Err(e) = fs::create_dir_all(dest_dir) {
            eprintln!("Fehler beim Erstellen des Zielverzeichnisses: {}", e);
            std::process::exit(1); // Beendet das Build-Skript mit einem Fehlercode
        }
    }

    // Überprüfen, ob das Quellverzeichnis existiert
    if src_dir.exists() {
        // Gehe durch alle Dateien im Quellverzeichnis und kopiere sie ins Zielverzeichnis
        for entry in fs::read_dir(src_dir).unwrap_or_else(|e| {
            eprintln!("Fehler beim Lesen des Verzeichnisses {}: {}", src_dir.display(), e);
            std::process::exit(1); // Beendet das Build-Skript mit einem Fehlercode
        }) {
            let entry = entry.unwrap();
            let src_path = entry.path();
            let dest_path = dest_dir.join(entry.file_name());

            // Wenn es eine Datei ist, dann kopiere sie
            if src_path.is_file() {
                if let Err(e) = fs::copy(&src_path, &dest_path) {
                    eprintln!("Fehler beim Kopieren der Datei {} nach {}: {}", src_path.display(), dest_path.display(), e);
                    std::process::exit(1); // Beendet das Build-Skript mit einem Fehlercode
                }
            }
        }
    } else {
        eprintln!("Quellverzeichnis {} existiert nicht", src_dir.display());
        std::process::exit(1); // Beendet das Build-Skript mit einem Fehlercode
    }

    // Sagt Cargo, dass das Skript neu ausgeführt werden soll, wenn sich das assets/ Verzeichnis ändert
    println!("cargo:rerun-if-changed=assets/");
}
