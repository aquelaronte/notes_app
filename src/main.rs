use notes_rust_io::{db, json};
use std::io;

fn main() {
    let mut option = String::new();

    println!("Bienvenido a notes app");
    println!("Seleccione un módulo para continuar (JSON por defecto)");
    println!("1 - JSON\n2 - DB");

    io::stdin()
        .read_line(&mut option)
        .expect("Error reading option");

    let option = match option.trim().parse::<u32>() {
        Err(_) => 1,
        Ok(data) => data,
    };

    match option {
        1 => json::init(),
        2 => db::init(),
        _ => json::init(),
    }
}
