use std::fs;
use std::path;
use std::process;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use crate::tamaowlchi::{Tamaowlchi, update_tamaowlchi_state};
use directories::BaseDirs;


const ENCRYPT_KEY: &str = match option_env!("ENCRYPT_KEY") {
    Some(v) => v,
    None => "default encrypt key for dev env",
};

pub fn load_current_tamaowlchi () -> Option<Tamaowlchi> {
    let config_file_path = get_config_file_path();

    if !config_file_path.exists() {
        return  None;
    }

    let tamaowlchi_config = fs::read_to_string(config_file_path).expect("Can't read config file."); // Return None instead
    
    parse_tamaowlchi_config(tamaowlchi_config).map(update_tamaowlchi_state)
}

pub fn save_tamaowlchi (tamaowlchi: &Tamaowlchi) {
    let config_file_path = get_config_file_path();

    let tamaowlchi_config = format_tamaowlchi_config(tamaowlchi);

    fs::write(config_file_path, tamaowlchi_config).expect("Can't save your tamaowlchi.");
}

fn get_config_file_path() -> path::PathBuf {
    let base_dir = BaseDirs::new().unwrap();
    let mut config_path = path::PathBuf::from(base_dir.config_dir());

    if !config_path.as_path().exists() {
        println!("Config path does not exist");
        match fs::create_dir(&config_path) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Could not create the file");
                process::exit(1);
            }
        }
    }

    config_path.push("tamaowlchi-config.txt");

    config_path
}

fn parse_tamaowlchi_config(config: String) -> Option<Tamaowlchi> {
    let mc = new_magic_crypt!(ENCRYPT_KEY, 256);
    let parsed_config = mc.decrypt_base64_to_string(config).expect("Error parsing the config file");

    serde_json::from_str(&parsed_config).unwrap()
}

fn format_tamaowlchi_config(tamaowlchi: &Tamaowlchi) -> String {
    let json_str = serde_json::to_string(tamaowlchi).unwrap();
    
    let mc = new_magic_crypt!(ENCRYPT_KEY, 256);
    mc.encrypt_str_to_base64(json_str)
}
