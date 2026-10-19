use crate::types::MonoError;
use zeroize::Zeroizing;

pub fn prompt_lock_password() -> Result<Zeroizing<String>, MonoError> {
    let password = rpassword::prompt_password("Enter password: ")?;

    if password.is_empty() {
        return Err(MonoError::InvalidInput(
            "Password cannot be empty".to_string(),
        ));
    }

    let confirmation = rpassword::prompt_password("Confirm password: ")?;

    if password != confirmation {
        return Err(MonoError::InvalidInput(
            "Passwords do not match".to_string(),
        ));
    }

    Ok(Zeroizing::new(password))
}

pub fn prompt_unlock_password() -> Result<Zeroizing<String>, MonoError> {
    Ok(Zeroizing::new(rpassword::prompt_password(
        "Enter password: ",
    )?))
}
