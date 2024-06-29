
use std::io::Read;

use serde::{Deserialize, Serialize};
use serde_json::Error;

use crate::json::json_file;

#[derive(Debug, Deserialize, Serialize)]

pub struct Note {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created_at: i64,
}

pub fn get_notes() -> Result<Vec<Note>, Error> {
    let mut file = json_file::get_file();
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)
        .expect("Failed getting notes");

    let notes: Vec<Note> = serde_json::from_str(&file_contents)?;
    Ok(notes)
}

pub fn create_note(note: Note) {
    let mut notes = get_notes().expect("Error getting notes");

    notes.push(note);
    let file_content = serde_json::to_string_pretty(&notes).expect("Failed to serialize notes");

    json_file::write_file(file_content).expect("Failed to save file content");
    return;
}

pub fn delete_note(id: &str) {
    let notes = get_notes().expect("Error getting notes");

    let notes: Vec<Note> = notes.into_iter().filter(|note| note.id != id).collect();

    let file_content = serde_json::to_string_pretty(&notes).expect("Failed to serialize notes");

    json_file::write_file(file_content).expect("Failed to save file content");
    return;
}

pub fn update_note(id: &str, title: &str, description: &str) {
    let notes = get_notes().expect("Error getting notes");
    let mut new_notes: Vec<Note> = vec![];

    for note in notes {
        if note.id != id {
            new_notes.push(note);
            continue;
        }

        let new_title = if title.len() == 0 {
            note.title
        } else {
            String::from(title)
        };

        let new_description = if description.len() == 0 {
            note.description
        } else {
            String::from(description)
        };

        let new_note = Note {
            id: note.id,
            title: new_title,
            description: new_description,
            created_at: note.created_at,
        };

        new_notes.push(new_note);
    }

    let file_content = serde_json::to_string_pretty(&new_notes).expect("Failed to serialize notes");

    json_file::write_file(file_content).expect("Failed to save file content");
    return;
}
