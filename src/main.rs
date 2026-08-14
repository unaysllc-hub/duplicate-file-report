use duplicate_file_report::scan;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut json = false;
    let mut minimum_size = 1_u64;
    let mut roots = Vec::new();
    let mut arguments = env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--json" => json = true,
            "--min-size" => {
                minimum_size = arguments
                    .next()
                    .unwrap_or_else(|| usage("--min-size requires a byte count"))
                    .parse()
                    .unwrap_or_else(|_| usage("--min-size must be an integer"));
            }
            value if value.starts_with('-') => usage("unknown option"),
            value => roots.push(PathBuf::from(value)),
        }
    }
    if roots.is_empty() {
        usage("provide at least one directory");
    }
    let report = scan(&roots, minimum_size);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("report serialization failed")
        );
    } else {
        println!(
            "Scanned {} files; found {} duplicate groups representing {} avoidable bytes.",
            report.scanned_files,
            report.groups.len(),
            report.duplicate_bytes
        );
        for group in &report.groups {
            println!("\n{} bytes  sha256:{}", group.size, group.sha256);
            for path in &group.files {
                println!("  {}", path.display());
            }
        }
        for warning in &report.warnings {
            eprintln!("warning: {warning}");
        }
    }
}

fn usage(message: &str) -> ! {
    eprintln!("duplicate-file-report: {message}");
    eprintln!("Usage: duplicate-file-report [--json] [--min-size BYTES] <directory>...");
    std::process::exit(2);
}
