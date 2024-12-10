use log::info;

pub fn greet(name: &str) -> String {
    info!("path: {}", name);
    format!("Hello {}!", name)
}