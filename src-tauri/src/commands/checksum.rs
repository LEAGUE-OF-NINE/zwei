use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use regex::Regex;

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

async fn get_manifest() -> Result<HashMap<String, FileInfo>, Box<dyn Error>> {
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

    Ok(file_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manifest_fetch() -> Result<(), Box<dyn Error>> {
        let manifest = get_manifest().await?;
        manifest.iter().for_each(|(key, _)| println!("File: {}", key));

        // Test that we got some data
        assert!(!manifest.is_empty(), "Manifest should not be empty");

        // Test for specific known files
        assert!(manifest.contains_key("GameAssembly.dll"), "Should contain GameAssembly.dll");
        assert!(manifest.contains_key("LimbusCompany.exe"), "Should contain LimbusCompany.exe");

        // Test specific file properties
        if let Some(game_assembly) = manifest.get("GameAssembly.dll") {
            assert!(game_assembly.size > 0, "GameAssembly.dll should have non-zero size");
            assert_eq!(game_assembly.sha.len(), 40, "SHA hash should be 40 characters long");
        }

        // Print all entries for debugging
        for (name, info) in &manifest {
            println!("File: {}", name);
            println!("  Size: {}", info.size);
            println!("  SHA:  {}", info.sha);
            println!();
        }

        Ok(())
    }
}
