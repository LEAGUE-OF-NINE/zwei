use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{write, Display, Formatter};
use std::fs::File;
use std::io::{Read, Stderr, Write};
use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;
use std::str::FromStr;
use regex::Regex;
use sha1::{Digest, Sha1};
use crate::commands::checksum::ManifestError::{FileDoesNotExist, ImpossibleError, MismatchedContent, MismatchedType, UnknownFile};

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

#[derive(Debug, PartialEq)]
enum ManifestError {
    UnknownFile,
    FileDoesNotExist,
    MismatchedType{ wanted_dir: bool },
    MismatchedContent,
    ImpossibleError,
}

impl Display for ManifestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnknownFile => write!(f, "File is not list in the manifest"),
            MismatchedType { wanted_dir: true } => write!(f, "Expected directory but got a file"),
            MismatchedType { wanted_dir: false } => write!(f, "Expected file but got a directory"),
            MismatchedContent => write!(f, "File does not match the checksum"),
            FileDoesNotExist => write!(f, "File should exist but does not"),
            ImpossibleError => write!(f, "Unknown error"),
        }
    }

}

impl Error for ManifestError {}

fn calculate_checksum_while<F>(path: PathBuf, mut process: F) -> Result<String, Box<dyn Error>>
where
    F: FnMut(&[u8]) -> Result<(), Box<dyn Error>>,
{
    // Open the file
    let mut file = File::open(&path)?;

    // Create a SHA-1 hasher
    let mut hasher = Sha1::new();

    // Read the file in chunks
    let mut buffer = [0u8; 8192]; // You can adjust the chunk size as needed
    loop {
        let bytes_read = file.read(&mut buffer)?;
        process(&buffer[..bytes_read])?;
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


fn calculate_checksum(path: PathBuf) -> Result<String, Box<dyn Error>> {
    calculate_checksum_while(path, |_| { Ok(() )})
}

impl VersionManifest {

    pub fn check_is_up_to_date(&self, game_dir: &PathBuf) -> Result<bool, Box<dyn Error>> {
        let catalog = "LimbusCompany_Data/StreamingAssets/aa/catalog.json";
        match self.check_file(game_dir, catalog, |_| { Ok(()) }) {
            Ok(_) => Ok(true),
            Err(e) if e.downcast_ref::<ManifestError>() == Some(&MismatchedContent) => Ok(false),
            Err(e) => Err(e)
        }
    }

    pub fn copy_to_folder(&self, src_dir: &PathBuf, dst_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
        // create directories
        for (name, info) in &self.0 {
            if info.is_folder() {
                let path = dst_dir.join(name);
                std::fs::create_dir_all(path)?;
            }
        }

        // copy files while verifying integrity
        for (name, info) in &self.0 {
            if info.is_folder() {
                continue;
            }

            let src = src_dir.join(name.clone());
            let dst = dst_dir.join(name);
            let mut dst_file = File::create(&dst)?;

            // verify integrity of src while copying to dst
            let checksum = calculate_checksum_while(src, move |chunk| {
                if chunk.len() > 0 {
                    dst_file.write_all(chunk)?;
                } else {
                    dst_file.flush()?;
                }
                Ok(())
            })?;

            if checksum != info.sha {
                return Err(MismatchedContent.into());
            }
        }

        Ok(())
    }

    pub fn check_file<F>(&self, game_dir: &PathBuf, child: &str, process: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&[u8]) -> Result<(), Box<dyn Error>>,
    {
        let info = match self.0.get(&child.to_string()) {
            None => return Err(UnknownFile.into()),
            Some(info) => info,
        };

        let path = game_dir.join(child);
        if !path.exists() {
            return Err(FileDoesNotExist.into());
        }

        if path.is_dir() != info.is_folder() {
            return Err(MismatchedType {
                wanted_dir: info.is_folder(),
            }.into())
        }

        if path.is_dir() {
            if info.size == 0 {
                return Ok(())
            }
        }

        if path.is_file() {
            let metadata = path.metadata()?;
            if metadata.size() != info.size {
                return Err(MismatchedContent.into());
            }
            if calculate_checksum_while(path, process)? != info.sha {
                return Err(MismatchedContent.into());
            }
            return Ok(())
        }

        Err(ImpossibleError.into())
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
        let src = PathBuf::from("Limbus Company/");
        let dst = PathBuf::from("/tmp/limbus-test/");
        manifest.copy_to_folder(&src, &dst)?;
        Ok(())
    }


}
