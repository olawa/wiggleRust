// wig file
// fixedStep chrom=chr10 start=9968 step=1
// 0.157
use std::fs::File;
use std::io::{self, BufRead};
use std::{path::Path, string};
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
extern crate regex;



fn read_wig(file_name: &str) ->  HashMap<String,String> {
    let file = File::open(file_name).expect("file not found!");
    let reader = BufReader::new(file);
    let re = regex::Regex::new(r"=| ").unwrap();
    let mut wiggle: HashMap<String,String> = HashMap::with_capacity(1000); 
    let mut chr: String ="chr1".to_string() ;
    let mut pos: i32 = 0;
    let mut step: usize=1;

    for line in reader.lines() {
        if let Ok(l) = line { 
            if l.starts_with("fixed") {
                let words = re.split(&l).collect::<Vec<&str>>();
                chr = words.get(2).unwrap().to_string();
                pos = words.get(4).unwrap().parse::<i32>().expect("Invalid input");
            } else {
                pos += 1;
                let key = format!("{}:{}",&chr, pos); 
                wiggle.entry(key).or_insert(l.to_string());
                
            }
        }

    }
    wiggle
}

pub(crate) fn main() {
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");
      
    let mut wiggle = read_wig(&file);
    for (key, value) in wiggle {
        println!("{}: {}", key, value);
    }
}
