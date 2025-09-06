use crate::config::load_config;
use serde::Serialize;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

#[derive(Serialize)]
struct IndexData {
    images: Vec<Image>,
}

#[derive(Serialize)]
struct Image {
    file_name: String,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut env = minijinja::Environment::new();
    let images_path = Path::new(&config.directories.images);
    let output_path = Path::new(&config.directories.output);
    let mut images = Vec::new();

    for entry in WalkDir::new(images_path).into_iter().filter_map(|e| e.ok()) {
        println!("{:?}", entry);
        if entry.path().is_dir() {
            handle_directory(&entry)?;
        } else {
            images.push(Image {
                file_name: entry.file_name().to_string_lossy().to_string(),
            });
            // handle_file(&entry)?;
        }
    }

    let index_html = fs::read_to_string(format!("{}/index.html", &config.directories.templates))
        .expect("No template/index.html found in working directory.");
    env.add_template_owned("index.html", index_html).unwrap();
    let index_html_template = env.get_template("index.html").unwrap();
    let index_html_rendered = index_html_template
        .render(minijinja::context! {
            title => &config.theme.title,
            root_path => &config.server.root_path,
            data => IndexData { images },
        })
        .unwrap();
    fs::write(output_path.join("index.html"), index_html_rendered)?;

    Ok(())
}

fn handle_directory(entry: &DirEntry) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn handle_file(entry: &DirEntry) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
