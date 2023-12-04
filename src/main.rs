use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use time::Duration;

use std::{fs::File, io::Read, thread::sleep, time::SystemTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GpsPoint {
    latitude: f64,
    longitude: f64,
    timestamp: u64,
}

#[derive(Parser)]
#[command(name = "ReplayGPS")]
#[command(version = "1.0")]
#[command(about = "Replay GPS track", long_about = None)]
struct Cli {
    /// input file
    #[arg(long)]
    file: String,
}

fn main() {
    // Read JSON file
    ctrlc::set_handler(move || {
        std::process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");

    let args = Cli::parse();
    let file_path = args.file;
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read file");

    // Parse JSON into Vec<GpsPoint>
    let gps_points: Vec<GpsPoint> =
        serde_json::from_str(&file_content).expect("Error parsing JSON");

    println!(
        "{} {} {}",
        "Replaying".green(),
        gps_points.len().to_string().bold(),
        "gps points".green()
    );

    let start_timestamp = gps_points.first().map_or(0, |point| point.timestamp);
    let end_timestamp = gps_points.last().map_or(0, |point| point.timestamp);

    let duration = Duration::milliseconds((end_timestamp - start_timestamp).try_into().unwrap());

    println!("{} {}", "Track duration: ".green(), duration.to_string());
    println!("{}", "Replaying...".blue());
    // Replay and send data
    for (index, point) in gps_points.iter().enumerate() {
        if index > 0 {
            // Calculate time difference and sleep
            let time_diff = gps_points[index].timestamp - gps_points[index - 1].timestamp;
            sleep(std::time::Duration::from_millis(time_diff));
        }
        let mut new_point = point.clone();
        let current_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_millis() as u64;

        new_point.timestamp = current_timestamp;

        // Use the point data to format the output
        let output_data = serde_json::to_string(&new_point).expect("Error formatting JSON");

        // Choose between MQTT or HTTP based on your configuration
        // For example, send_to_mqtt(&output_data);
        // or send_to_http(&output_data);
        println!("{}", output_data);
    }
}
