use log::info;

pub fn greet(name: &str) -> String {
    info!("Receive data: {}", name);
    format!("Hello {}!", name)
}