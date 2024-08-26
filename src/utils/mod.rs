use std::fs;
use std::io::{self, Read, Write};

pub fn read_file_to_vec(path: &str) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut file = fs::File::open(path)?;
    file.read_to_end(&mut data)?;
    Ok(data)
}

pub fn write_file(path: &str, content: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content)?;
    Ok(())
}
