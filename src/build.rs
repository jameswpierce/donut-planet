use crate::config::load_config;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    println!("config: {:?}", config);
    Ok(())
}
