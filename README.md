# Duplicate File Report

A safe, report-only Rust utility that finds duplicate files by grouping candidates by size and then comparing SHA-256 hashes. It never deletes, moves, renames or edits files.

## Install and run

```bash
cargo build --release
./target/release/duplicate-file-report --min-size 1024 ~/Documents
```

Produce machine-readable output:

```bash
duplicate-file-report --json ./photos ./backups > duplicates.json
```

## Why two stages?

Files with unique byte lengths cannot be duplicates, so the scanner hashes only size groups with multiple candidates. Hashing streams data in 64 KiB blocks to avoid loading large files into memory.

The `duplicate_bytes` value counts all but one copy in each duplicate group. Review every path yourself before taking any action; this program intentionally has no delete option.

## Test

```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

## License

MIT License.
