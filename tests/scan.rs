use duplicate_file_report::scan;
use std::fs;
use tempfile::tempdir;

#[test]
fn groups_same_content_and_calculates_avoidable_bytes() {
    let directory = tempdir().unwrap();
    fs::write(directory.path().join("first.txt"), b"same content").unwrap();
    fs::write(directory.path().join("second.txt"), b"same content").unwrap();
    fs::write(directory.path().join("different.txt"), b"other content").unwrap();
    let report = scan(&[directory.path().to_path_buf()], 1);
    assert_eq!(report.scanned_files, 3);
    assert_eq!(report.groups.len(), 1);
    assert_eq!(report.groups[0].files.len(), 2);
    assert_eq!(report.duplicate_bytes, 12);
}

#[test]
fn minimum_size_skips_hashing_small_files() {
    let directory = tempdir().unwrap();
    fs::write(directory.path().join("one"), b"a").unwrap();
    fs::write(directory.path().join("two"), b"a").unwrap();
    let report = scan(&[directory.path().to_path_buf()], 2);
    assert!(report.groups.is_empty());
}
