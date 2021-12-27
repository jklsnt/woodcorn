use std::io::prelude::*;    // i have no clue how to write rust
use std::fs::File;

use std::collections::HashMap;

struct MysteryDatastructureItem {
    n: HashMap<char, MysteryDatastructureItem>,
    m: u8,
}
impl MysteryDatastructureItem {
    fn new() -> Self {
        Self { n: HashMap::new(), m: 0 }
    }
    fn trav(&mut self, c: char) -> &mut MysteryDatastructureItem {
        //println!("adding node {}", c);
        self.n.entry(c).or_insert_with(|| { println!("inserting {}", c); MysteryDatastructureItem::new() })
    }
}

fn add_to_dict(mut r: MysteryDatastructureItem, word: &str, mask: u8) -> MysteryDatastructureItem {
    let mut c = &mut r;
    for letter in word.chars() { c = c.trav(letter); }
    c.m = mask;
    r
}

fn check_word(r: MysteryDatastructureItem, word: &str) -> Option<(String, String)> {

    None     // TODO
}

fn main() -> Result<(), std::io::Error> {
    let mut rf = File::open("../problem/wordlist.txt")?;
    let mut dict = String::new();
    rf.read_to_string(&mut dict)?;

    let dict: Vec<Vec<&str>> = dict
        .trim().split("\n").skip(1)
        .map(|s| s.trim().split(",").collect())
        .collect();

    for entry in dict {
        let word = entry[0];
        let pos = entry[1].parse::<u8>().expect(&format!("failed to parse mask {}", entry[1]));
        //println!("{}: {}", word, pos);
    }

    let mut t = MysteryDatastructureItem::new();

    for i in 1..1_000_000 {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        let line = line.trim_end();
        t = add_to_dict(t, line, 1);
        println!("{}", line);
    }

    println!("Hello, world!");
    Ok(())
}
