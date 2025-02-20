use log::debug;
use std::io::{self, Write};
use std::process::Command;

pub fn get_info() -> Result<String, io::Error> {
    // Costruisci il comando
    let mut command = Command::new("sudo");
    command
        .arg("/home/giulio/.local/bin/ryzenadj")
        .arg("--info");

    //dbg!("Esecuzione del comando: {:?}", &command);

    // Esegui il comando e cattura l'output
    let output = command.output()?;

    // Verifica se il comando è stato eseguito con successo
    if output.status.success() {
        // Converte l'output in una stringa
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        //dbg!("Output del comando: {}", &stdout);
        Ok(stdout)
    } else {
        // Se c'è stato un errore, cattura l'errore
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        dbg!("Errore durante l'esecuzione del comando: {}", &stderr);
        Err(io::Error::new(io::ErrorKind::Other, stderr))
    }
}

pub(crate) fn reset_fast_limit(target: u32) {
    // Costruisci il comando
    let mut command = Command::new("sudo");
    command.arg("/home/giulio/.local/bin/ryzenadj")
    .arg(format!("--fast-limit={}", target));
    
    //dbg!("Esecuzione del comando: {:?}", &command);

    // Esegui il comando e cattura l'output
    let output = command.output().unwrap();

    // Verifica se il comando è stato eseguito con successo
    if output.status.success() {
        // Converte l'output in una stringa
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        dbg!("Output del comando: {}", &stdout);
        
    } else {
        // Se c'è stato un errore, cattura l'errore
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        dbg!("Errore durante l'esecuzione del comando: {}", &stderr);
    }
}
pub(crate) fn reset_slow_limit(target: u32) {
    // Costruisci il comando
    let mut command = Command::new("sudo");
    command.arg("/home/giulio/.local/bin/ryzenadj")
    .arg(format!("--slow-limit={}", target));
    
    //dbg!("Esecuzione del comando: {:?}", &command);

    // Esegui il comando e cattura l'output
    let output = command.output().unwrap();

    // Verifica se il comando è stato eseguito con successo
    if output.status.success() {
        // Converte l'output in una stringa
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        dbg!("Output del comando: {}", &stdout);
        
    } else {
        // Se c'è stato un errore, cattura l'errore
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        dbg!("Errore durante l'esecuzione del comando: {}", &stderr);
    }
}
