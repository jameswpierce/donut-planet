use crate::config::load_config;
use serde::Serialize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize)]
struct IndexData {
    images: Vec<Image>,
}

#[derive(Serialize)]
struct Image {
    path: String,
    file_name: String,
    extension: String,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut env = minijinja::Environment::new();
    let images_path = Path::new(&config.directories.images);
    let output_path = Path::new(&config.directories.output);
    let mut images = Vec::new();

    for entry in WalkDir::new(images_path).into_iter().filter_map(|e| e.ok()) {
        println!("{entry:?}");
        if entry.path().is_dir() {
            println!("directory =^w^=");
        } else {
            images.push(Image {
                path: entry.path().as_os_str().to_string_lossy().to_string(),
                file_name: entry.file_name().to_string_lossy().to_string(),
                extension: entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or_default()
                    .to_string(),
            });
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
