use serde_derive::{Serialize, Deserialize};
use std::path::Path;
use std::fs::{File};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub oauth_consumer_key : String, 
    pub oauth_consumer_secret : String,
    pub oauth_token : String,
    pub oauth_token_secret : String,
}

impl Config {
    pub fn read(path_file: &Path) -> Option<Config> {
        let mut file = match File::open(path_file) {
            Ok(f) => f,
            Err(_) => return None,
        };
        serde_json::from_reader(&mut file).ok()
    }
}