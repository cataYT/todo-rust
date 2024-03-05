use std::{fs::File, io::Write};

pub fn add(item: &str) -> std::io::Result<()> {
    let mut file: File = File::create("todo.txt")?;
    file.write_all(item.as_bytes())?;
    file.flush()?;
    Ok(())

}

pub fn remove() {
    
}

pub fn list() {

}