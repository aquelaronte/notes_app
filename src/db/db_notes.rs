use super::db_file::get_db;

pub struct Note {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

pub fn create_note(title: &str, description: &str) {
    let mut connection = get_db();

    let tx = connection
        .transaction()
        .expect("Error getting connection transaction");

    tx.execute(
        "
        INSERT INTO note(title, description) VALUES(?1, ?2);
    ",
        [title, description],
    )
    .expect("Error when trying to create a note into DB");

    tx.commit().expect("Error when commiting changes");
}

pub fn get_notes() -> Vec<Note> {
    let connection = get_db();

    let mut stmt = connection
        .prepare("SELECT id, title, description, created_at FROM note")
        .expect("Error when preparing query");

    let note_iter = stmt
        .query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .expect("Error when trying to parse data from db");

    let mut notes: Vec<Note> = vec![];

    for note in note_iter {
        notes.push(note.unwrap());
    }

    notes
}

pub fn delete_note(id: i64) {
    let mut connection = get_db();

    let tx = connection
        .transaction()
        .expect("Error getting connection transaction");

    tx.execute(
        "
        DELETE FROM note WHERE id = ?1;
    ",
        [id],
    )
    .expect("Error when trying to execute delete note into DB");

    tx.commit()
        .expect("Error when commiting deleting operation");
}

pub fn update_note(id: &str, title: &str, description: &str) {
    let include_title = if title.len() > 0 { true } else { false };
    let include_description = if description.len() > 0 { true } else { false };

    let mut connection = get_db();

    let tx = connection.transaction().expect("Error getting transaction");

    if include_title && include_description {
        tx.execute(
            "
            UPDATE note SET title = ?2, description = ?3 WHERE id = ?1;
        ",
            [id, title, description],
        )
        .expect("Error when trying to update note");
    } else if include_title {
        tx.execute(
            "
            UPDATE note SET title = ?2 WHERE id = ?1;
        ",
            [id, title],
        )
        .expect("Error when trying to update note");
    } else if include_description {
        tx.execute(
            "
            UPDATE note SET description = ?2 WHERE id = ?1;
        ",
            [id, description],
        )
        .expect("Error when trying to update note");
    }

    tx.commit().expect("Error when trying to commit changes");
}
