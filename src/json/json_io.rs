use std::io::{self};

use chrono::Utc;
use uuid::Uuid;

use crate::json::json_notes::{self, Note};

pub fn main() {
    let mut option = String::new();
    println!("Iniciando Notes App módulo JSON...\n");

    loop {
        println!("Ingresa una opción\n\t1 - Ver notas guardadas\n\t2 - Crear nota\n\t3 - Eliminar una nota\n\t4 - Editar nota\n\t5 - Salir");
        io::stdin()
            .read_line(&mut option)
            .expect("Error reading option line");

        let parsed_option = match option.trim().parse::<u32>() {
            Err(_) => {
                println!("Opción inválida, reintentando...");
                option.clear();
                continue;
            }
            Ok(value) => value,
        };

        match parsed_option {
            1 => show_saved_notes(),
            2 => create_note(),
            3 => delete_note(),
            4 => update_note(),
            5 => break,
            _ => {
                println!("Opción inválida, reintentando...");
                option.clear();
                continue;
            }
        }
        option.clear();
    }
}

fn delete_note() {
    let mut id = String::new();

    println!("Ingresa el id de una nota");

    io::stdin().read_line(&mut id).expect("Error reading id");

    json_notes::delete_note(id.trim());
}

fn create_note() {
    let v4 = Uuid::new_v4();
    let id = v4.to_string();
    let mut title = String::new();
    let mut description = String::new();
    let created_at = Utc::now().timestamp();

    println!("Nota con id: {}", id);
    println!("Ingrese un título");

    io::stdin()
        .read_line(&mut title)
        .expect("Error reading title");

    println!("Ingrese una descripción");

    io::stdin()
        .read_line(&mut description)
        .expect("Error reading description");

    let note = Note {
        id,
        title: String::from(title.trim()),
        description: String::from(description.trim()),
        created_at,
    };

    json_notes::create_note(note);
}

fn show_saved_notes() {
    println!("\nNotas:");
    let notes = json_notes::get_notes().expect("Error getting notes");

    for note in notes {
        println!(
            "Nota:\n\tid: {}\n\ttitle: {}\n\tdescription: {}\n\tcreated_at: {}",
            note.id, note.title, note.description, note.created_at
        )
    }
}

fn update_note() {
    let mut id = String::new();
    let mut title = String::new();
    let mut description = String::new();

    println!("Ingresa el id de la nota a editar");
    io::stdin()
        .read_line(&mut id)
        .expect("Error reading note id");

    println!("Ingresa el nuevo título (dejar vacío si no se quiere editar)");
    io::stdin()
        .read_line(&mut title)
        .expect("Error modifying note title");

    println!("Ingresa la nueva descripción (dejar vacío si no se quiere editar)");
    io::stdin()
        .read_line(&mut description)
        .expect("Error modifying note description");

    json_notes::update_note(id.trim(), title.trim(), description.trim());
}
