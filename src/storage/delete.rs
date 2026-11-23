use crate::types::MonoError;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Removes every path, continuing past failures so one bad file doesn't leave
/// the rest behind. Missing files are ignored; failed paths are reported together.
pub fn delete_files(paths: Vec<&Path>) -> Result<(), MonoError> {
    let mut failures: Vec<String> = Vec::new();
    let mut kind = None;

    for path in paths {
        match fs::remove_file(path) {
            Ok(()) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => {
                kind.get_or_insert(err.kind());
                failures.push(format!("{} ({err})", path.display()));
            }
        }
    }

    match kind {
        None => Ok(()),
        Some(kind) => Err(MonoError::Io(std::io::Error::new(
            kind,
            format!("failed to remove: {}", failures.join(", ")),
        ))),
    }
}

pub fn shred_and_delete(paths: Vec<&Path>) -> Result<(), MonoError> {
    for path in paths {
        let mut file = OpenOptions::new().write(true).open(path)?;

        let metadata = file.metadata()?;
        let length = metadata.len() as usize;

        let zeros = vec![0u8; length];
        file.write_all(&zeros)?;
        file.flush()?;

        file.sync_all()?;

        std::fs::remove_file(path)?;
    }

    Ok(())
}
