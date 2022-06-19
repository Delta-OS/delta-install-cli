use docopt::Docopt;
use online::check;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{fs, io};
extern crate serde_json;

use serde_json::Value;

#[async_std::main]
async fn main() {
    const USAGE: &'static str = "
    Usage: 
        delta-install --config=<config_path> <destination>
        delta-install -h | --help
    
    Options:
        -h, --help  Show this help message
        --config,   Specify configuration file for your version of Delta
    ";

    let args = Docopt::new(USAGE)
        .and_then(|d| d.argv(std::env::args().into_iter()).parse())
        .unwrap_or_else(|e| e.exit());

    let config_file_path = args.get_str("--config");

    if !Path::new(config_file_path).exists() {
        return println!("Please enter a valid configuration file path... Exiting.");
    }

    let data = fs::read_to_string(config_file_path)
        .expect("Error while reading config file. Is file valid ?");

    let json_data: Value = serde_json::from_str(&data).unwrap();

    check_key("packages", &json_data);
    check_key("sources", &json_data);

    let device = &json_data["device"];
    let architecture = &json_data["arch"];
    let hostname = &json_data["hostname"];
    let packages = &json_data["packages"];
    let ppa_array = &json_data["ppa_array"];
    let sources = &json_data["sources"];

    println!("You are going to install Delta GNU/Linux project with this parameters:\n\t- Device: {}\n\t- Arch: {}\n\t- Hostname: {}\n\t- Packages: {}\n\t- PPA: {}", device.as_str().unwrap(), architecture.as_str().unwrap(), hostname.as_str().unwrap(),packages, ppa_array.to_string());
    println!(
        "Do you accept installing Delta GNU/Linux with this parameters into {} ? (Y/N) ",
        args.get_str("<destination>")
    );

    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Error while reading user input");

    if response.trim() == "N" || response.trim() == "n" {
        return;
    } else if response.trim() == "Y" || response.trim() == "y" {
        println!("Checking if parameters are correct...");
        if device != "desktop" && device != "server" && device != "pi" {
            return println!(
                "Current device '{}' is not available in our repositories. Check the wiki.",
                device
            );
        }
        if architecture != "amd64" && architecture != "arm64" {
            return println!(
                "Current architecture {} is not available in our repositories. Check the wiki.",
                architecture
            );
        }
        println!("Checking if user is connected to internet...\n");
        assert!(check(None).await.is_ok());
        println!("Starting installation...");
        let command = format!(
            "{} {} {} {} {}",
            format!("debootstrap --arch={} --components=main,restricted,universe,multiverse",architecture),
            format!("--include {}", packages.to_string().replace("[", "").replace("]", "").replace("\"", "")),
            "jammy",
            args.get_str("<destination>"),
            "http://archive.ubuntu.com/ubuntu/"
        );
        println!("{}", command);
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute install command. Is debootstrap installed ?");
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo \"{}\" > {}/etc/hostname", hostname, args.get_str("<destination>")))
            .output()
            .expect("Error when updating hostname.");

        std::fs::copy(sources.to_string(), format!("{}/etc/apt/sources.list", args.get_str("<destination>"))).expect("Error happened when trying to copy sources file into the new system.");
    }
}

fn check_key(key_name: &str, json_data: &Value) {
    if !json_data.get(key_name).is_some() {
        println!(
            "{} key isn't detected in config file. Please write it, check the wiki.",
            key_name
        );
        std::process::exit(1);
    }
}
