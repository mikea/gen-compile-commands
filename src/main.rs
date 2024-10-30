//! Simple `compile_commands.json` generator.

use clap::Parser;
use serde::Serialize;
use std::{
    fs::{self},
    path::PathBuf,
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Args {
    /// Root project directory, all files in `compile_commands.json` will be specified relative
    /// to this path.
    #[arg(long)]
    root: PathBuf,

    /// Path to the `compile_flags.txt`. Used for every file in `compile_commands.json`.
    #[arg(long)]
    compile_flags: PathBuf,

    /// Path to the resulting `compile_commands.json`.
    #[arg(long)]
    out: PathBuf,

    /// Directory to scan for source files.
    #[arg(long)]
    src_dir: Vec<PathBuf>,

    /// Source file extensions to include in `compile_commands.json`.
    #[arg(long, default_values_t=["c++".to_string(), "cc".to_string()])]
    ext: Vec<String>,

    /// Path to the compiler.
    #[arg(long, default_value="/usr/bin/clang++")]
    compiler: String,
}

fn main() {
    let args = Args::parse();

    let flags = fs::read_to_string(&args.compile_flags).unwrap_or_else(|err| {
        panic!(
            "error reading compile flags from {}: {err}",
            args.compile_flags.display()
        )
    });

    let mut flags: Vec<String> = flags
        .split('\n')
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.to_owned())
            }
        })
        .collect();

    // first argument needs to be the compiler
    flags.push(args.compiler.clone());

    let mut entries = vec![];
    for dir in args.src_dir {
        assert!(
            dir.strip_prefix(&args.root).is_ok(),
            "'--root {}' is not a prefix of '--src_dir {}'",
            args.root.to_string_lossy(),
            dir.to_string_lossy(),
        );

        for entry in WalkDir::new(dir).into_iter().filter_entry(|e| {
            e.file_type().is_dir()
                || args.ext.contains(
                    &e.path()
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                )
        }) {
            let entry = entry.unwrap();
            if entry.file_type().is_dir() {
                continue;
            }

            entries.push(CompilationDatabaseEntry {
                directory: args.root.to_string_lossy().to_string(),
                file: entry
                    .path()
                    .strip_prefix(&args.root)
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                arguments: flags.clone(),
                output: None,
            });
        }
    }

    fs::write(args.out, serde_json::to_string_pretty(&entries).unwrap()).unwrap();
}

/// <https://clang.llvm.org/docs/JSONCompilationDatabase.html/>
#[derive(Serialize)]
struct CompilationDatabaseEntry {
    directory: String,
    arguments: Vec<String>,
    file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
}
