use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default)]
struct Device {
    device_id: String,
    name: String,
    location: String,
    serial: String,
    features: Vec<String>,
}

fn parse_device(block: &str) -> Option<Device> {
    let mut device = Device::default();

    for line in block.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();

        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim();

        let value = parts[1].trim();

        match key {
            "device_id" => device.device_id = value.to_string(),
            "name" => device.name = value.to_string(),
            "location" => device.location = value.to_string(),
            "serial" => device.serial = value.to_string(),
            "features" => {
                let cleaned = value.trim_matches(&['[', ']'][..]);
                device.features = cleaned.split(',').map(|s| s.trim().to_string()).collect();
            }
            _ => {}
        }
    }

    Some(device)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Использование: smart_helper <config_file> <поисковая_фраза>");
        return Ok(());
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut buffer = String::new();
    let mut matches = Vec::new();
    let query = args[2].to_lowercase();

    for line in reader.lines() {
        let line = line?;

        if line.trim() == "---" {
            if let Some(device) = parse_device(&buffer) {
                if device.name.to_lowercase().contains(&query) {
                    matches.push(device);
                }
            }
            buffer.clear();
        } else {
            buffer.push_str(&line);
            buffer.push('\n');
        }
    }

    if !buffer.trim().is_empty() {
        if let Some(device) = parse_device(&buffer) {
            if device.name.to_lowercase().contains(&query) {
                matches.push(device);
            }
        }
    }

    if matches.is_empty() {
        println!("🚫 Ничего не найдено по запросу '{}'", query);
    } else {
        println!("🔍 Найдено совпадений {}:", matches.len());
        for device in matches {
            println!("🔶 Название: {}", device.name);
            println!("🔶 ID: {}", device.device_id);
            println!("🔶 Местоположение: {}", device.location);
            println!("🔶 Серийный номер: {}", device.serial);
            println!("🔶 Возможности: {}", device.features.join(", "));
        }
    }

    Ok(())
}
