use std::io::prelude::*;    // i have no clue how to write rust
use std::fs::File;
use std::default::Default;

use std::collections::HashMap;

#[derive(Default)]
struct MysteryDatastructureItem {
    n: HashMap<char, MysteryDatastructureItem>,
    m: u8,
}
impl MysteryDatastructureItem {
    fn new() -> Self {
        Self { n: HashMap::new(), m: 0 }
    }
    fn add(&mut self, c: char) -> &mut MysteryDatastructureItem {
        //self.n.entry(c).or_insert_with(|| { println!("    ins {}", c); MysteryDatastructureItem::new() })
        self.n.entry(c).or_default()
    }
    fn get(&self, c: &char) -> Option<&MysteryDatastructureItem> {
        self.n.get(c)
    }
}

fn insert_dict(mut r: MysteryDatastructureItem, word: &str, m: u8) -> MysteryDatastructureItem {
    let mut c = &mut r;
    for letter in word.chars() { c = c.add(letter); }
    c.m = m;
    r
}

fn query_dict(mut c: &MysteryDatastructureItem, word: &str) -> u8 {
    for letter in word.chars() {
        if let Some(x) = c.get(&letter) {
            c = x;
        } else {
            return 0;
        }
    }
    c.m
}

fn check_word(r: &MysteryDatastructureItem, word: &str) -> Vec<(String, String)> {
    let mut c = r;
    let mut ret = Vec::new();
    for (i, letter) in word.chars().enumerate() {
        c = match c.get(&letter) {
            Some(x) => x,
            None => return ret,
        };
        if c.m > 0 {
            let m2 = query_dict(r, &word[i+1..]);
            if m2 > 0 && c.m | m2 == 3 {
                //println!("{}: {} {} ({} {})", word, word[..i+1].to_owned(), word[i+1..].to_owned(), c.m, m2);
                ret.push((word[..i+1].to_owned(), word[i+1..].to_owned()));
            }
        }
    }
    ret
}

fn main() -> Result<(), std::io::Error> {
    let mut rf = File::open("../problem/wordlist.txt")?;
    let mut dict = String::new();
    rf.read_to_string(&mut dict)?;

    let dict: Vec<Vec<&str>> = dict
        .trim().split("\n").skip(1)
        .map(|s| s.trim().split(",").collect())
        .collect();
    println!("input read");

    let mut t = MysteryDatastructureItem::new();
    for entry in dict.iter() {
        let word = entry[0];
        let pos = entry[1].parse::<u8>().expect(&format!("failed to parse POS {}", entry[1]));
        t = insert_dict(t, word, pos);
    }
    println!("dictionary built");

    //for i in 1..1_000_000 {
    //    let mut buf = String::new();
    //    std::io::stdin().read_line(&mut buf)?;
    //    let buf = buf.trim();
    //    println!("got {} for {}", query_dict(&t, &buf), buf)
    //}

    let mut ret = Vec::new();
    for entry in dict.iter() {
        if (query_dict(&t, entry[0]) & 1) == 1 {
            ret.append(&mut check_word(&t, entry[0]));
        }
    }
    println!("words found");
    for (word1, word2) in ret {
        println!("{} {}", word1, word2);
    }
    Ok(())
}
