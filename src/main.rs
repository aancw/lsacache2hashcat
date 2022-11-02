// Copyright (c) 2022 Petruknisme
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate clap;
extern crate colored;

use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Parser)]
#[command(name = "lsacache2hashcat")]
#[command(author = "Petruknisme <me@petruknisme.com>")]
#[command(version = "1.0")]
#[command(about = "Give me lsadump::cache from mimikatz, I will transform it to DCC2 Hashcat compatible. Useful for dumping so many credentials caches", long_about = None)]
struct Cli {
    /// Lsadump cache output from mimikatz
    #[clap(short, long)]
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let file = cli.file;
    let re_iter = Regex::new(
        r"\(([^)]+)\)"
    ).unwrap();
    let re_user = Regex::new(
        r"([^\\]+$)"
    ).unwrap();
    let re_hash = Regex::new(
        r"([^ :]+$)"
    ).unwrap();
    let mut iteration: &str;
    let mut user: &str;
    let mut dcc2_hash: &str;

    if let Ok(lines) = read_lines(file) {

        for line in lines {
            if let Ok(i) = line {
                if i.contains("Iteration") {
                    let text_re = re_iter.captures(&i).unwrap();
                    iteration = text_re.get(1).map_or("", |m| m.as_str());                
                }
                if i.contains("User"){
                    let text_re = re_user.captures(&i).unwrap();
                    user = text_re.get(1).map_or("", |m| m.as_str()); 
                }
                if i.contains("MsCacheV2"){
                    let text_re = re_hash.captures(&i).unwrap();
                    dcc2_hash = text_re.get(1).map_or("", |m| m.as_str()); 
                }

            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}