use std::io;
use zeroize::Zeroizing;

pub fn prompt_lock_password() -> io::Result<Zeroizing<String>> {
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

    Ok(Zeroizing::new(password))
}

pub fn prompt_unlock_password() -> io::Result<Zeroizing<String>> {
    Ok(Zeroizing::new(rpassword::prompt_password(
        "Enter password: ",
    )?))
}
