use std::str::Chars;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use serde_json;

fn main() -> Result<(), std::io::Error> {
    // Read the input file
    let mut file = File::open("./words_alpha.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    // Split into words
    let words:Vec<String> = contents
        .split("\n")
        .map(|i| i.replace("\r", ""))
        .collect();

    // Create a hashmap of words
    let mut dict:HashMap<String, bool> = HashMap::new();
    for term in &words {
        dict.insert(term.clone(), true);
    }

    // Create a hashmap of halfzies, an a vector of results
    let mut result:Vec<(String,String)> = vec![];

    // Iterate to find halfzies
    for term in &words {
        let chars:Chars = term.chars();
        for i in 1..term.len() {
            let first_half = chars.clone().take(i).collect::<String>();
            let second_half = chars.clone().skip(i).collect::<String>();
            let reverse = format!("{}{}", second_half, first_half);

            if dict.contains_key(&reverse) {
                result.push((first_half, second_half));
            }
        }
    }

    let serialized = serde_json::to_string(&result).unwrap();

    let mut file = File::create("woodchuck.json")?;
    file.write_all(serialized.as_bytes())?;

    return Ok(());
}
