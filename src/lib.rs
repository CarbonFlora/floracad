#![allow(dead_code, unused_variables)]

pub mod horizontal_calculation;
pub mod vertical_calculation;
pub mod angle_system;
pub mod horizontal_create;
pub mod vertical_create;
pub mod sight_distance;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::Result;

pub fn parse_text_file(file_path: &str) -> Result<HashMap<String, String>> {
    let input = File::open(file_path)?;
    let buffered = BufReader::new(input);
    let mut arguments = HashMap::new();

    for line in buffered.lines().flatten() {
        let i = line.split_once('=');
        match i {
            None => continue,
            Some(args) => arguments.insert(args.0.to_owned(), args.1.to_owned()),
        };
    }
    Ok(arguments)
}






