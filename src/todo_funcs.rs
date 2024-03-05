use std::{fs::File, io::{BufRead, BufReader, Write}, fs::OpenOptions};

pub fn add(item: &str) -> std::io::Result<()> {
    let mut file: File = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.txt")?;
    file.write_all(item.as_bytes())?;
    file.flush()?;
    Ok(())

}

pub fn remove(item: &str) -> std::io::Result<()> {
    let mut file_content: String = String::new(); 
    {
        let file: File = File::open("todo.txt")?;
        let reader: BufReader<File> = BufReader::new(file);
        for line in reader.lines() {
            let line: String = line?;
            file_content.push_str(&line);
            file_content.push('\n');
        }
    }

    let modified_contents: String = file_content.replace(item, "");

    let mut file: File = File::create("todo.txt")?;
    file.write_all(modified_contents.as_bytes())?;

    Ok(())
}

pub fn list() -> std::io::Result<String> {
    let mut result: String = String::new();
    let file: File = File::open("todo.txt")?;
    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        result.push_str(&line?);
        result.push('\n');
    }
    Ok(result)
}