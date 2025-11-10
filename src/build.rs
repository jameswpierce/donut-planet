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
    resized_src: String,
    original_src: String,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut env = minijinja::Environment::new();
    let mut images = Vec::new();

    for entry in WalkDir::new(&config.directories.images)
        .into_iter()
        .filter_map(Result::ok)
    {
        let extension = entry
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default()
            .to_lowercase();
        if entry.path().is_dir() {
            println!("directory =^w^=");
        } else if ["jpg".to_string(), "gif".to_string()].contains(&extension) {
            let original_file_name = entry
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let resized_file_name = format!("800x{original_file_name}");
            let resized_file_directory = Path::new("processed-images/");
            if !resized_file_directory.exists() {
                fs::create_dir(&resized_file_directory)?;
            }
            let original_file_path = resized_file_directory.join(&original_file_name);
            let resized_file_path = resized_file_directory.join(&resized_file_name);
            if !resized_file_path.exists() {
                println!("resizing {original_file_name}");
                dbg!(&resized_file_path);

                let image = image::open(&entry.path()).unwrap();
                let ratio: f32 = 800.0 / image.width() as f32;
                let scaled_height = (image.height() as f32 * ratio) as u32;
                image.save(&original_file_path)?;
                image
                    .resize(800, scaled_height, image::imageops::FilterType::Triangle)
                    .save(&resized_file_path)?;
            }

            images.push(Image {
                resized_src: resized_file_name,
                original_src: original_file_name,
            });
        }
    }

    let index_html = fs::read_to_string(config.directories.templates.join("index.html"))
        .expect("No template/index.html found in working directory.");
    env.add_template_owned("index.html", index_html).unwrap();
    let index_html_template = env.get_template("index.html").unwrap();
    let index_html_rendered = index_html_template
        .render(minijinja::context! {
            title => &config.theme.title,
            root_path => &config.server.root_path,
            data => IndexData { images },
            config => &config,
        })
        .unwrap();
    fs::write(
        &config.directories.output.join("index.html"),
        index_html_rendered,
    )?;

    Ok(())
}
