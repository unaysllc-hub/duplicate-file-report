use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct DuplicateGroup {
    pub size: u64,
    pub sha256: String,
    pub files: Vec<PathBuf>,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub scanned_files: usize,
    pub duplicate_bytes: u64,
    pub groups: Vec<DuplicateGroup>,
    pub warnings: Vec<String>,
}

pub fn scan(roots: &[PathBuf], minimum_size: u64) -> Report {
    let mut by_size: BTreeMap<u64, Vec<PathBuf>> = BTreeMap::new();
    let mut warnings = Vec::new();
    let mut scanned_files = 0;

    for root in roots {
        for entry in WalkDir::new(root).follow_links(false) {
            match entry {
                Ok(entry) if entry.file_type().is_file() => match entry.metadata() {
                    Ok(metadata) if metadata.len() >= minimum_size => {
                        by_size.entry(metadata.len()).or_default().push(entry.into_path());
                        scanned_files += 1;
                    }
                    Ok(_) => scanned_files += 1,
                    Err(error) => warnings.push(format!("{}: {error}", entry.path().display())),
                },
                Ok(_) => {}
                Err(error) => warnings.push(error.to_string()),
            }
        }
    }

    let mut groups = Vec::new();
    for (size, candidates) in by_size.into_iter().filter(|(_, files)| files.len() > 1) {
        let mut by_hash: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
        for path in candidates {
            match hash_file(&path) {
                Ok(hash) => by_hash.entry(hash).or_default().push(path),
                Err(error) => warnings.push(format!("{}: {error}", path.display())),
            }
        }
        for (sha256, mut files) in by_hash.into_iter().filter(|(_, files)| files.len() > 1) {
            files.sort();
            groups.push(DuplicateGroup { size, sha256, files });
        }
    }
    groups.sort_by(|left, right| right.size.cmp(&left.size).then_with(|| left.sha256.cmp(&right.sha256)));
    let duplicate_bytes = groups
        .iter()
        .map(|group| group.size * (group.files.len() as u64 - 1))
        .sum();
    Report { scanned_files, duplicate_bytes, groups, warnings }
}

fn hash_file(path: &Path) -> io::Result<String> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(hex::encode(digest.finalize()))
}
