use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{write, Display, Formatter};
use std::fs::File;
use std::io::{Read, Stderr};
use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;
use std::str::FromStr;
use regex::Regex;
use sha1::{Digest, Sha1};
use crate::commands::checksum::ManifestError::{MismatchedContent, MismatchedType, UnknownFile};

const FOLDER_SHA: &str = "0000000000000000000000000000000000000000";

#[derive(Debug)]
struct FileInfo {
    size: u64,
    sha: String,
}

impl FileInfo {

    fn is_folder(&self) -> bool {
        self.sha == FOLDER_SHA
    }

}

struct VersionManifest(HashMap<String, FileInfo>);

#[derive(Debug)]
enum ManifestError {
    UnknownFile,
    MismatchedType{ wanted_dir: bool },
    MismatchedContent,
}

impl Display for ManifestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnknownFile => write!(f, "File is not list in the manifest"),
            MismatchedType { wanted_dir: true } => write!(f, "Expected directory but got a file"),
            MismatchedType { wanted_dir: false } => write!(f, "Expected file but got a directory"),
            MismatchedContent => write!(f, "File does not match the checksum"),
        }
    }

}

impl Error for ManifestError {}

fn calculate_checksum(path: PathBuf) -> Result<String, Box<dyn Error>> {
    // Open the file
    let mut file = File::open(&path)?;

    // Create a SHA-1 hasher
    let mut hasher = Sha1::new();

    // Read the file in chunks
    let mut buffer = [0u8; 1024]; // You can adjust the chunk size as needed
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Finalize the hash and get the hexadecimal representation
    let result = hasher.finalize();
    let hex_digest = format!("{:X}", result); // Format as uppercase hexadecimal

    Ok(hex_digest)
}

impl VersionManifest {

    pub fn check_file(&self, game_dir: &PathBuf, child: &str) -> Result<(), Box<dyn Error>> {
        let info = match self.0.get(&child.to_string()) {
            None => return Err(UnknownFile.into()),
            Some(info) => info,
        };

        let path = game_dir.join(child);
        if path.is_dir() != info.is_folder() {
            return Err(MismatchedType {
                wanted_dir: info.is_folder(),
            }.into())
        }

        if path.is_file() {
            let metadata = path.metadata()?;
            if metadata.size() != info.size {
                return Err(MismatchedContent.into());
            }
            if calculate_checksum(path)? != info.sha {
                return Err(MismatchedContent.into());
            }
        }

        Ok(())
    }

}

async fn get_manifest() -> Result<VersionManifest, Box<dyn Error>> {
    let url = "https://api.lethelc.site/limbus-manifest.txt";
    let response = reqwest::get(url).await?.text().await?;

    let mut file_map = HashMap::new();

    let header = Regex::new(r"^\s*Size\s*Chunks\s*File SHA\s*Flags Name\s*$").unwrap();
    let mut after_header = false;

    for line in response.lines() {
        if after_header {
            if line.len() < 70 {
                continue
            }

            let size_str = line[0..14].trim();
            let sha = line[22..63].trim().to_string();
            let file_name = line[69..].trim();
            let size = u64::from_str(size_str)?;

            file_map.insert(file_name.to_string(), FileInfo{ size, sha, });
        }

        after_header |= header.is_match(line);
    }

    Ok(VersionManifest(file_map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manifest_fetch() -> Result<(), Box<dyn Error>> {
        let manifest = get_manifest().await?;
        manifest.0.iter().for_each(|(key, _)| println!("File: {}", key));

        // Test that we got some data
        assert!(!manifest.0.is_empty(), "Manifest should not be empty");

        // Test for specific known files
        assert!(manifest.0.contains_key("GameAssembly.dll"), "Should contain GameAssembly.dll");
        assert!(manifest.0.contains_key("LimbusCompany.exe"), "Should contain LimbusCompany.exe");

        // Test specific file properties
        if let Some(game_assembly) = manifest.0.get("GameAssembly.dll") {
            assert!(game_assembly.size > 0, "GameAssembly.dll should have non-zero size");
            assert_eq!(game_assembly.sha.len(), 40, "SHA hash should be 40 characters long");
        }

        // Print all entries for debugging
        for (name, info) in &manifest.0 {
            println!("File: {}", name);
            println!("  Size: {}", info.size);
            println!("  SHA:  {}", info.sha);
            println!();
        }

        Ok(())
    }


    #[tokio::test]
    async fn test_check_file() -> Result<(), Box<dyn Error>> {
        let manifest = get_manifest().await?;
        let path = PathBuf::from("Limbus Company/");

        for (name, _) in &manifest.0 {
            let ok = manifest.check_file(&path, name).is_ok();
            println!("{}, ok: {}", name, ok);
        }

        Ok(())
    }


}
