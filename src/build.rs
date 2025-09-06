use crate::config::load_config;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let images_path = Path::new(&config.directories.images);
    let output_path = Path::new(&config.directories.output);

    for entry in WalkDir::new(images_path).into_iter().filter_map(|e| e.ok()) {
        println!("{:?}", entry);
        if entry.path().is_dir() {
            handle_directory(&entry)?;
        } else {
            handle_file(&entry)?;
        }
    }

    Ok(())
}

fn handle_directory(entry: &DirEntry) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn handle_file(entry: &DirEntry) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
