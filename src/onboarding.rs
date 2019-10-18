use super::banner::BANNER;
use super::config::ClientConfig;
use crate::config::LOCALHOST;
use std::io::stdin;

pub struct Onboarding {}

impl Onboarding {
    pub fn new() -> Onboarding {
        Onboarding {}
    }

    pub fn is_done(&mut self) -> bool {
        let config = ClientConfig::new();
        match config.get_or_build_paths() {
            Ok(paths) => paths.config_file_path.exists(),
            Err(_e) => false,
        }
    }

    pub fn board(&mut self) -> Result<(), failure::Error> {
        println!("{}", BANNER);

        let config = ClientConfig::new();
        match config.get_or_build_paths() {
            Ok(paths) => {
                println!(
                    "Config will be saved to {}",
                    paths.config_file_path.display()
                );
            }
            Err(e) => {
                panic!(e);
            }
        }

        println!("\nHow to get setup:\n");

        let instructions = [
            "Go to the Spotify dashboard - https://developer.spotify.com/dashboard/applications",
            "Click `Create a Client ID` and create an app",
            "Now click `Edit Settings`",
            &format!("Add `{}` to the Redirect URIs", LOCALHOST),
            "You are now ready to authenticate with Spotify!",
        ];

        let mut number = 1;
        for item in instructions.iter() {
            println!("  {}. {}", number, item);
            number += 1;
        }

        let mut client_id: String;
        let mut client_secret: String;

        match Onboarding::ask_required_token("Client ID") {
            Ok(token) => {
                client_id = token;
            }
            Err(e) => {
                panic!(e);
            }
        }

        match Onboarding::ask_required_token("Client Secret") {
            Ok(token) => {
                client_secret = token;
            }
            Err(e) => {
                panic!(e);
            }
        }

        let mut config_yml = ClientConfig {
            client_id: client_id,
            client_secret: client_secret,
            device_id: None,
        };

        config_yml.save()
    }

    fn ask_required_token(name: &str) -> Result<String, failure::Error> {
        let mut token = String::new();
        let mut token_required_shown = false;
        println!("\nEnter your {}: ", name);
        stdin().read_line(&mut token)?;

        while token.trim() == "" {
            if token_required_shown {
                println!("\r");
            }
            token_required_shown = true;

            println!("\r");
            println!("\nA {} is required!", name);
            println!("\nEnter your {}: ", name);
            stdin().read_line(&mut token)?;
        }

        Ok(token.trim().to_string())
    }
}
