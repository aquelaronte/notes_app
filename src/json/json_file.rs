use std::{
    fs::{self, File, OpenOptions},
    io::{Error, Write},
};

const JSON_FILE_PATH: &str = ".notes.json";

fn file_exists() -> bool {
    fs::metadata(JSON_FILE_PATH).is_ok()
}

fn create_file() -> File {
    let mut file = File::create(JSON_FILE_PATH).expect("Error creating file");
    file.write_all("[]".as_bytes()).expect("Error writing file");
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(JSON_FILE_PATH)
        .expect("Error opening created file")
}

pub fn get_file() -> File {
    if file_exists() {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(JSON_FILE_PATH)
            .expect("Error getting file")
    } else {
        create_file()
    }
}

pub fn write_file(content: String) -> Result<(), Error> {
    if !file_exists() {
        create_file();
    }

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .truncate(true)
        .open(JSON_FILE_PATH)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}
