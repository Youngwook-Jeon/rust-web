use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn get_handle() -> Result<File, String> {
    let file_path = env::var("JSON_STORE_PATH").unwrap_or("./tasks.json".to_string());
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)
        .map_err(|e| format!("Error opening file: {}", e))?;
    Ok(file)
}
