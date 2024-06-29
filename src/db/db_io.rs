use std::io;

use crate::db::db_notes;

pub fn main() {
    let mut option = String::new();
    println!("Iniciando Notes App módulo DB...\n");

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
            1 => io_get_notes(),
            2 => io_create_note(),
            3 => io_delete_note(),
            4 => io_update_note(),
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

fn io_update_note() {
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

    db_notes::update_note(id.trim(), title.trim(), description.trim());
}

fn io_delete_note() {
    let mut id = String::new();

    println!("Ingresa el id de una nota");

    io::stdin().read_line(&mut id).expect("Error reading id");

    let id: i64 = id.trim().parse().expect("Failed parsing option");

    db_notes::delete_note(id);
}

fn io_create_note() {
    let mut title = String::new();
    let mut description = String::new();

    println!("Ingrese un título");

    io::stdin()
        .read_line(&mut title)
        .expect("Error reading title");

    println!("Ingrese una descripción");

    io::stdin()
        .read_line(&mut description)
        .expect("Error reading description");

    db_notes::create_note(title.trim(), description.trim());
}

fn io_get_notes() {
    println!("Notas:");
    let notes = db_notes::get_notes();

    for note in notes {
        println!(
            "Nota:\n\tid: {}\n\ttitle: {}\n\tdescription: {}\n\tcreated_at: {}",
            note.id, note.title, note.description, note.created_at
        )
    }
}
