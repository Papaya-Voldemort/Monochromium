use crate::types::MonoError;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn delete_files(paths: Vec<&Path>) -> Result<(), MonoError> {
    for path in paths {
        match fs::remove_file(path) {
            Ok(()) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => return Err(MonoError::Io(err)),
        }
    }

    Ok(())
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
