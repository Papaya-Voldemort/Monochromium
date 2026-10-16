use std::io;

pub fn prompt_lock_password() -> io::Result<String> {
    let password = rpassword::prompt_password("Enter password: ")?;

    if password.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Password cannot be empty",
        ));
    }

    let confirmation = rpassword::prompt_password("Confirm password: ")?;

    if password != confirmation {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Passwords do not match",
        ));
    }

    Ok(password)
}

pub fn prompt_unlock_password() -> io::Result<String> {
    rpassword::prompt_password("Enter password: ")
}
