use crate::database::{UpdateType, update_note};

pub struct Output {
    pub old: String,
    pub new: String
}

pub async fn edit(
    conn: libsql::Connection,
    note_id: u32,
    headless: bool,
    append: bool,
    overwrite: bool,
    text: String,
) -> Result<Output, libsql::Error> {
    let new = text.clone();

    let update_type = if append {
        UpdateType::Append
    } else if overwrite {
        UpdateType::Overwrite
    } else {
        return Err(libsql::Error::Misuse(
            "Must specify either append or overwrite".into(),
        ));
    };

    let old = update_note(conn, note_id, text, UpdateType::Overwrite).await?;


    Ok(Output {
        old,
        new,
    })
}
