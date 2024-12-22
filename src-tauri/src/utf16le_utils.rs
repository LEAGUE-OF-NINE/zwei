/// Sandboxie.ini config uses UTF16-LE for its encoding... These utils are here to help read and write to the config.
use std::{fs::File, io::Read};

use byteorder::{LittleEndian, WriteBytesExt};

pub fn read_utf16le_file_to_u16(file_path: &str) -> std::io::Result<Vec<u16>> {
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // Ensure the buffer length is even
    if buf.len() % 2 != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File does not contain valid UTF-16LE data: odd number of bytes",
        ));
    }

    // Convert raw bytes to UTF-16LE `u16` units
    let utf16_data = buf
        .chunks(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    Ok(utf16_data)
}

pub fn write_u16_as_utf16le(file_path: &str, utf16_data: &[u16]) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    for &unit in utf16_data {
        file.write_u16::<LittleEndian>(unit)?;
    }
    Ok(())
}

pub fn append_utf16le(file_path: &str, new_content: &str) -> std::io::Result<()> {
    let mut utf16_data = read_utf16le_file_to_u16(file_path)?;

    // Convert the new content (UTF-8) to UTF-16
    let new_utf16: Vec<u16> = new_content.encode_utf16().collect();

    // Append the new content
    utf16_data.extend(new_utf16);

    // Write back the updated content
    write_u16_as_utf16le(file_path, &utf16_data)
}

pub fn add_directive(file_path: &str, directive: &str) -> std::io::Result<()> {
    // Read the file content
    let utf16_data = read_utf16le_file_to_u16(file_path)?;

    // Convert UTF-16 data to a UTF-8 string
    let content = String::from_utf16(&utf16_data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Check if the directive already exists
    if content.lines().any(|line| line.trim() == directive) {
        return Ok(()); // Directive already exists, no need to add
    }

    // Append the new directive to the content
    let updated_content = format!("{}\n{}", content, directive);

    // Convert the updated content back to UTF-16
    let updated_utf16: Vec<u16> = updated_content.encode_utf16().collect();

    // Write back the updated content
    write_u16_as_utf16le(file_path, &updated_utf16)
}

pub fn remove_directive(file_path: &str, directive: &str) -> std::io::Result<()> {
    // Read the file content
    let utf16_data = read_utf16le_file_to_u16(file_path)?;

    // Convert UTF-16 data to a UTF-8 string
    let content = String::from_utf16(&utf16_data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Filter out the unwanted directive
    let updated_content = content
        .lines()
        .filter(|line| !line.trim().eq(directive))
        .collect::<Vec<_>>()
        .join("\n");

    // Convert the updated content back to UTF-16
    let updated_utf16: Vec<u16> = updated_content.encode_utf16().collect();

    // Write back the updated content
    write_u16_as_utf16le(file_path, &updated_utf16)
}
