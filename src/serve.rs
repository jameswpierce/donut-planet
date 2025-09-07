use crate::config::load_config;

pub fn run(dev_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    if dev_mode {
        println!("starting donut planet server in dev mode...");
        println!(
            "{}:{}{}",
            config.server.domain, config.server.port, config.server.root_path
        );
        Ok(())
    } else {
        println!("starting donut planet server...");
        Ok(())
    }
}
