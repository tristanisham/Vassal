use std::env;
use std::fs;
use std::path::Path;
// use std::fs::File;
// use dirs;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;

mod core;

use crate::core::parse::{SettingsFile};

fn main() {
    println!("Vassal {}", env!("CARGO_PKG_VERSION"));
    let argv: Vec<String> = env::args().collect();
    let flag = &argv[1].as_str();
    
    if cfg!(windows) {
        
        let game_dir = match flag {
            &"--dir" => argv[2].to_string(),
            _ => "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Crusader Kings III".to_string(),
        };
        let target: String = format!("{}/AppData/Local/Vassal/", env!("USERPROFILE"));
        //TODO: Add config file

        let program_files = Path::new(&target);
        match program_files.exists() {
            true => {}
            false => match make_setup_files(false, Some(&target)) {
                Ok(()) => println!("Success"),
                Err(e) => eprintln!("{}", e),
            },
        }

        // let documents = match dirs::document_dir() {
        //     Some(x) => x
        // };

        let ck3 = Path::new(&game_dir);
        if ck3.exists() {
            println!("Crusader Kings III Located.");
            let checksum_manifest = SettingsFile::new(format!("{}/game/checksum_manifest.txt", &ck3.to_string_lossy()));
            // checksum_manifest.display_manifest();
            checksum_manifest.make_json(&target);
            
        } else {
            eprintln!("Error finding game at: {}", game_dir);
        }
    }
}

fn make_setup_files(c: bool, target: Option<&str>) -> std::io::Result<()> {
    if !c {
        if let Some(t) = target {
            fs::create_dir_all(t)?;
            let mut config_file = File::create(format!("{}/config.json", t))?;
            todo!("Build and write config file using ConfigFile struct");
        }

    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    game_path: String,
    app_path: String,
}